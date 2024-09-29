from __future__ import annotations

from contextlib import suppress
from typing import Type, TYPE_CHECKING

from alive_progress import alive_it

from binary_file_parser.errors import CompressionError, ParsingError, VersionError
from binary_file_parser.types.parseable import Parseable
from binary_file_parser.types.byte_stream import ByteStream
from binary_file_parser.types.version import Version
from binary_file_parser.utils import TabbedStringIO

if TYPE_CHECKING:
    from binary_file_parser.retrievers import Retriever, RetrieverCombiner, RetrieverRef


class BaseStruct(Parseable):
    """
    Base class for defining a file format as a structure. Most methods of this class are protected to be hidden from the
    end user API
    """
    __slots__ = "_struct_ver"

    _retrievers: list[Retriever] = []
    _refs: list[RetrieverRef] = []
    _combiners: list[RetrieverCombiner] = []

    @classmethod
    def _add_retriever(cls, retriever: Retriever):
        cls._retrievers.append(retriever)

    @classmethod
    def _add_ref(cls, ref: RetrieverRef):
        cls._refs.append(ref)

    @classmethod
    def _add_combiner(cls, combiner: RetrieverCombiner):
        cls._combiners.append(combiner)

    def __init__(self, struct_ver: Version = Version((0,)), initialise_defaults: bool = True, **retriever_inits):
        """
        :param struct_ver: The struct version to create
        :param initialise_defaults:
            If set to false, skip initialisation of struct values from default. This is only set to false when reading
            a file
        :param retriever_inits:
            Use this to provide initial values to retrievers
        """
        self._struct_ver = struct_ver

        size = 0
        if not initialise_defaults:
            super().__init__(size)
            return

        for retriever in self._retrievers:
            if not retriever.supported(struct_ver):
                continue

            init = retriever_inits.get(retriever.p_name, None)
            if init is None:
                init = retriever.from_default(self)
            setattr(self, retriever.p_name, init)

            # size += retriever.dtype._size
        super().__init__(size)

    def __init_subclass__(cls, **kwargs):
        """
        Note: subclasses of BaseStruct which intend to act as ABCs need to override this
        """
        cls._retrievers, BaseStruct._retrievers = cls._retrievers.copy(), []
        cls._refs, BaseStruct._refs = cls._refs.copy(), []
        cls._combiners, BaseStruct._combiners = cls._combiners.copy(), []

    @property
    def _struct(self):
        return self

    @property
    def struct_ver(self) -> Version:
        """The struct_ver of this struct used when retrievers are versioned"""
        return self._struct_ver

    @classmethod
    def _get_version(cls, stream: ByteStream, struct_ver: Version = Version((0,))) -> Version:
        """
        If defined, the struct will be versioned and the retrievers in the struct which are not supported will be
        skipped during read/write and will raise a VersionError on access.

        :param stream: The stream to read the struct version from
        :param struct_ver: The struct version of the parent

        :return: A tuple of ints indicating the version eg: v1.47 should return (1, 47)

        :raises VersionError: - When unimplemented
        """
        raise VersionError("Un-versioned File")

    @classmethod
    def _decompress(cls, bytes_: bytes) -> bytes:
        """
        If remaining_compressed is set to True in a Retriever, this method is used to decompress the bytes remaining
        before they are read and parsed into a struct object

        :param bytes_: The remaining (compressed) bytes of the compressed section of the struct

        :return: decompressed bytes

        :raises CompressionError: - When unimplemented
        """
        raise CompressionError(
            "Unable to read object from file. "
            "A Structure with a compressed section needs to implement 'decompress' classmethod."
        )

    @classmethod
    def _compress(cls, bytes_: bytes) -> bytes:
        """
        If remaining_compressed is set to True in a Retriever, this method is used to compress the bytes before they are
        written to file from the struct object

        :param bytes_: The remaining (uncompressed) bytes of the compressed section of the struct

        :return: compressed bytes

        :raises CompressionError: - When unimplemented
        """
        raise CompressionError(
            "Unable to write object to file. "
            "A Structure with a compressed section needs to implement 'compress' classmethod."
        )

    @classmethod
    def _from_stream(
        cls, stream: ByteStream, *, struct_ver: Version = Version((0,)), strict: bool = False,
        show_progress: bool = False
    ) -> BaseStruct:
        """
        Create a struct object from a ByteStream

        :param stream: The stream to create the struct object from
        :param struct_ver: The version of the structure to create. Overwritten if `get_version` is defined
        :param strict: Raise an error if struct parsing finishes successfully but the stream has left over bytes
        :param show_progress: When true, display a progress bar

        :return: An instance of a subtype of BaseStruct
        """
        with suppress(VersionError):
            struct_ver = cls._get_version(stream, struct_ver)

        instance = cls(struct_ver = struct_ver, initialise_defaults = False)
        retriever_ls = cls._retrievers
        if show_progress:
            retriever_ls = alive_it(
                retriever_ls,
                dual_line = True,
                title = "         Reading File",
                stats = False,
                finalize = lambda bar: bar.title("Finished Reading File")
            )

        for retriever in retriever_ls:
            if show_progress:
                retriever_ls.text = f"            -> {retriever.p_name.title().replace('_', ' ')}"
            if retriever.remaining_compressed:
                stream = stream.__class__.from_bytes(cls._decompress(stream.remaining()))
            retriever.from_stream(instance, stream)

        file_len = len(stream.content)

        if stream.progress != file_len and strict:
            raise ParsingError(
                f"{file_len - stream.progress} bytes are left after parsing all retrievers successfully"
            )

        return instance

    @classmethod
    def _from_bytes(
        cls, bytes_: bytes, *, struct_ver: Version = Version((0,)), strict = False,
        show_progress: bool = False, stream_cls: Type[ByteStream] = ByteStream
    ) -> BaseStruct:
        """
        Create a struct object from bytes

        :param bytes_: The bytes to create the struct object from
        :param struct_ver: The version of the structure to create. Overwritten if `get_version` is defined
        :param strict: Raise an error if struct parsing finishes successfully but there are unused bytes left over
        :param show_progress: When true, display a progress bar

        :return: An instance of a subtype of BaseStruct
        """
        stream = stream_cls.from_bytes(bytes_)
        return cls._from_stream(stream, struct_ver = struct_ver, strict = strict, show_progress = show_progress)

    @classmethod
    def _from_file(
        cls, file_name: str, *, file_version: Version = Version((0,)), strict = True,
        show_progress: bool = True, stream_cls: Type[ByteStream] = ByteStream
    ) -> BaseStruct:
        """
        Create a struct object from file

        :param file_name: The path of the file to create the struct object from
        :param file_version: The version of the structure to create. Overwritten if `get_version` is defined
        :param strict: Raise an error if struct parsing finishes successfully but the stream has left over bytes
        :param show_progress: When true, display a progress bar

        :return: An instance of a subtype of BaseStruct
        """
        stream = stream_cls.from_file(file_name)
        return cls._from_stream(stream, struct_ver = file_version, strict = strict, show_progress = show_progress)

    @classmethod
    def _from_compressed_file(
        cls, file_name: str, *, file_version: Version = Version((0,)), strict = True,
        show_progress: bool = True, stream_cls: Type[ByteStream] = ByteStream
    ) -> BaseStruct:
        """
        Create a struct object from file

        :param file_name: The path of the file to create the struct object from
        :param file_version: The version of the structure to create. Overwritten if `get_version` is defined
        :param strict: Raise an error if struct parsing finishes successfully but the stream has left over bytes
        :param show_progress: When true, display a progress bar

        :return: An instance of a subtype of BaseStruct
        """
        stream = stream_cls.from_file(file_name)
        stream = stream_cls.from_bytes(cls._decompress(stream.remaining()))
        return cls._from_stream(stream, struct_ver = file_version, strict = strict, show_progress = show_progress)

    def _to_bytes(self, *, show_progress = False) -> bytes:
        """
        Convert the struct object to bytes

        :param instance: The struct object to convert to bytes
        :param show_progress: When true, display a progress bar

        :return: bytes
        """
        length = len(self._retrievers)

        bytes_ = [b""] * length
        compress_idx = length
        retriever_ls = self._retrievers
        if show_progress:
            retriever_ls = alive_it(
                retriever_ls,
                dual_line = True,
                title = "         Writing File",
                stats = False,
                finalize = lambda bar: bar.title("Finished Writing File"),
            )

        for i, retriever in enumerate(retriever_ls):
            if show_progress:
                retriever_ls.text = f"            <- {retriever.p_name.title().replace('_', ' ')}"
            if retriever.remaining_compressed:
                compress_idx = i
            bytes_[i] = retriever.to_bytes(self)

        compressed = b""
        if compress_idx != length:
            compressed = self._compress(b"".join(bytes_[compress_idx:]))

        return b"".join(bytes_[:compress_idx]) + compressed

    def _to_file(self, file_name: str, *, show_progress: bool = True):
        """
        Write the bytes of the struct object to a file

        :param file_name: The name of the file to write to
        :param show_progress: When true, display a progress bar
        """
        with open(file_name, "wb") as file:
            file.write(self._to_bytes(show_progress = show_progress))

    def _to_compressed_file(self, file_name: str, *, show_progress: bool = True):
        """
        Write the bytes of the struct object to a file

        :param file_name: The name of the file to write to
        :param show_progress: When true, display a progress bar
        """
        with open(file_name, "wb") as file:
            file.write(self._compress(self._to_bytes(show_progress = show_progress)))

    def _diff(self, other: BaseStruct) -> dict[str, tuple | dict]:
        """
        Get a dictionary of retriever names to tuples values for which the two provided structs differ.

        :param other: The object to diff with

        :return: dictionary with differing retriever names as keys and their values as tuples.
        """
        if (
            not isinstance(other, BaseStruct)
            or type(self) is not type(other)
        ):
            raise TypeError(f"Cannot diff '{type(self)}' with an object of type '{type(other)}'")

        diff_retrievers: dict[str, tuple | dict] = {}
        for retriever in self._retrievers:
            diff = (getattr(self, retriever.p_name, None), getattr(other, retriever.p_name, None))
            match diff:
                case None, None: pass
                case _, None:
                    diff_retrievers[retriever.p_name] = ("...", None)
                case None, _:
                    diff_retrievers[retriever.p_name] = (None, "...")
                case val1, val2:
                    if isinstance(val1, BaseStruct):
                        sub_diff = val1._diff(val2)
                        if len(sub_diff) > 0:
                            diff_retrievers[retriever.p_name] = sub_diff
                    elif val1 != val2:
                        diff_retrievers[retriever.p_name] = diff

        return diff_retrievers

    def _dbg_repr(self) -> str:
        return self.__repr__(get = lambda obj, attr: getattr(obj, attr, None))

    def __repr__(self, ident: int = 0, get = getattr) -> str:
        builder = TabbedStringIO(ident)
        builder.write(f"{self.__class__.__name__}(")

        len_gt_zero = False
        with builder.tabbed():
            if self.struct_ver != Version((0,)):
                builder.writeln(f"struct_ver = {self.struct_ver},")
                len_gt_zero = True
            for retriever in self._retrievers:
                if not retriever.supported(self.struct_ver):
                    continue
                obj = get(self, retriever.p_name)
                if isinstance(obj, BaseStruct):
                    builder.writeln(f"{retriever.p_name} = {obj.__repr__(builder.ident, get)},")
                elif isinstance(obj, list):
                    builder.writeln(f"{retriever.p_name} = {_ls_repr(obj, builder.ident)},")
                else:
                    builder.writeln(f"{retriever.p_name} = {obj!r},")
                len_gt_zero = True

        if len_gt_zero:
            builder.writeln()

        builder.write(")")
        return builder.getvalue()

    def __eq__(self, other: object) -> bool:
        if (
            not isinstance(other, BaseStruct)
            or type(self) is not type(other)
            or self.struct_ver != other.struct_ver
        ):
            return False

        if id(self) == id(other):
            return True

        for retriever in self._retrievers:
            if not retriever.supported(self.struct_ver):
                continue
            if getattr(self, retriever.p_name) != getattr(other, retriever.p_name):
                return False
        return True

    # todo: write val <-> data (names) to file
    # todo: write hex (decompressed) to file
    # todo: file/header/decompressed in both hex/val <-> data

def _ls_repr(ls: list, ident: int = 0, get = getattr) -> str:
    builder = TabbedStringIO(ident)
    builder.write("[")

    with builder.tabbed():
        for item in ls:
            if isinstance(item, BaseStruct):
                builder.writeln(item.__repr__(builder.ident, get))
            elif isinstance(item, list):
                builder.writeln(_ls_repr(item, builder.ident, get))
            else:
                builder.writeln(repr(item))
            builder.write(",")

    if len(ls) > 0:
        builder.writeln()

    builder.write("]")
    return builder.getvalue()
