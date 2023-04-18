from __future__ import annotations

from io import StringIO
from contextlib import suppress
from typing import TYPE_CHECKING

from alive_progress import alive_it
from toolz import compose

from binary_file_parser.errors import CompressionError
from binary_file_parser.errors import ParsingError
from binary_file_parser.errors import VersionError
from binary_file_parser.types import ByteStream
from binary_file_parser.types import Parseable
from binary_file_parser.utils import indentify, Version

if TYPE_CHECKING:
    from binary_file_parser.retrievers.RetreiverRef import RetrieverRef
    from binary_file_parser.retrievers.Retriever import Retriever


class BaseStruct(Parseable):
    """
    Base class for defining a file format as a structure
    """
    __slots__ = "struct_ver", "parent"

    _retrievers: list[Retriever] = []
    _refs: list[RetrieverRef] = []

    @property
    def is_struct(self) -> bool:
        return True

    @property
    def retriever_name_value_map(self) -> dict[str]:
        map_ = {}
        for retriever in self._retrievers:
            map_[retriever.p_name] = getattr(self, retriever.p_name)
        return map_

    @classmethod
    def add_retriever(cls, retriever: Retriever):
        cls._retrievers.append(retriever)

    @classmethod
    def add_ref(cls, ref: RetrieverRef):
        cls._refs.append(ref)

    def __init_subclass__(cls, **kwargs):
        cls._retrievers, BaseStruct._retrievers = cls._retrievers.copy(), []

    @classmethod
    def from_default(cls, struct_ver: Version = Version((0,)), instance: BaseStruct = None) -> BaseStruct:
        """
        Create the object representing the file format using default values

        :param struct_ver: This is the version of the format that will be created
        :param instance: Initialise this object from defaults

        :return: An instance of a subtype of BaseStruct
        """
        instance = instance or cls() if not struct_ver else cls(struct_ver)
        for retriever in cls._retrievers:
            if not retriever.supported(instance.struct_ver):
                continue
            setattr(instance, retriever.p_name, retriever.from_default(instance))
        return instance

    def __init__(
        self,
        struct_ver: Version = Version((0,)),
        parent: BaseStruct = None,
        initialise_defaults: bool = True,
        **retriever_inits,
    ):
        """
        :param struct_ver: The struct version to create
        :param parent: If this struct is nested within another, define the containing struct as parent
        :param initialise_defaults:
            If set to false, skip initialisation of struct values from default. This is only set to false when reading
            a file
        """
        self.struct_ver = struct_ver
        self.parent = parent

        size = 0
        for retriever in self._retrievers:
            if not retriever.supported(struct_ver):
                continue

            if initialise_defaults:
                init = retriever_inits.get(retriever.p_name, None)
                setattr(self, retriever.p_name, init if init is not None else retriever.from_default(self))

            size += retriever.default.size if retriever.dtype.is_struct else retriever.dtype.size

        super().__init__(size)

    @classmethod
    def get_version(cls, stream: ByteStream, struct_ver: Version = Version((0,)), parent: BaseStruct = None) -> Version:
        """
        If defined, the struct will be versioned and values in the struct which are not supported in the version that is
        read will be skipped

        :param stream: The stream to read the struct version from
        :param struct_ver: The struct version of the parent
        :param parent:
        :return: A tuple of ints indicating the version eg: v1.47 should return (1, 47)
        :raises VersionError: - When unimplemented
        """
        raise VersionError("Un-versioned File")

    @classmethod
    def decompress(cls, bytes_: bytes) -> bytes:
        """
        If remaining_compressed is set to True in a Retriever, this method is used to decompress the bytes remaining
        before they are read and parsed into a struct object

        :param bytes_: The remaining (compressed) bytes of the compressed section of the struct
        :return: decompressed bytes
        :raises CompressionError: - When unimplemented
        """
        raise CompressionError(
            "Unable to read object from file. "
            "A Structure with compressed section needs to implement 'decompress' classmethod."
        )

    @classmethod
    def compress(cls, bytes_: bytes) -> bytes:
        """
        If remaining_compressed is set to True in a Retriever, this method is used to compress the bytes before they are
        written to file from the struct object

        :param bytes_: The remaining (uncompressed) bytes of the compressed section of the struct
        :return: compressed bytes
        :raises CompressionError: - When unimplemented
        """
        raise CompressionError(
            "Unable to write object to file. "
            "A Structure with compressed section needs to implement 'compress' classmethod."
        )

    @classmethod
    def from_stream(
        cls, stream: ByteStream, *, struct_ver: Version = Version((0,)), strict: bool = False,
        show_progress: bool = False, parent: BaseStruct = None
    ) -> BaseStruct:
        """
        Create a struct object from a ByteStream

        :param stream: The stream to create the struct object from
        :param struct_ver: The version of the structure to create. Overwritten if `get_version` is defined
        :param strict: Raise an error if struct parsing finishes successfully but the stream has left over bytes
        :param show_progress: When true, display a progress bar
        :param parent: If this struct is nested within another, define the containing struct as parent
        :return: An instance of a subtype of BaseStruct
        """
        with suppress(VersionError):
            struct_ver = cls.get_version(stream, struct_ver, parent)

        instance = cls(struct_ver, parent, initialise_defaults = False)
        retriever_ls = cls._retrievers
        if show_progress:
            retriever_ls = alive_it(
                retriever_ls,
                dual_line = True,
                title = f"         Reading File",
                stats = False,
                finalize = lambda bar: bar.title(f"Finished Reading File")
            )

        for retriever in retriever_ls:
            if show_progress:
                retriever_ls.text = f"            -> {retriever.p_name.title().replace('_', ' ')}"
            if retriever.remaining_compressed:
                stream = ByteStream.from_bytes(cls.decompress(stream.remaining()))
            retriever.from_stream(instance, stream)

        file_len = len(stream.content)

        if stream.progress != file_len and strict:
            raise ParsingError(
                f"{file_len - stream.progress} bytes are left after parsing all retrievers successfully:\n"
                f"{stream.remaining()}"
            )

        return instance

    @classmethod
    def from_bytes(cls, bytes_: bytes, *, struct_ver: Version = Version((0,)), strict = False) -> BaseStruct:
        """
        Create a struct object from bytes

        :param bytes_: The bytes to create the struct object from
        :param struct_ver: The version of the structure to create. Overwritten if `get_version` is defined
        :param strict: Raise an error if struct parsing finishes successfully but there are unused bytes left over
        :return: An instance of a subtype of BaseStruct
        """
        stream = ByteStream.from_bytes(bytes_)
        return cls.from_stream(stream, struct_ver = struct_ver, strict = strict)

    @classmethod
    def from_file(cls, file_name: str, *, file_version: Version = Version((0,)), strict = True) -> BaseStruct:
        """
        Create a struct object from file

        :param file_name: The path of the file to create the struct object from
        :param file_version: The version of the structure to create. Overwritten if `get_version` is defined
        :param strict: Raise an error if struct parsing finishes successfully but the stream has left over bytes
        :return: An instance of a subtype of BaseStruct
        """
        stream = ByteStream.from_file(file_name)
        return cls.from_stream(stream, struct_ver = file_version, strict = strict, show_progress = True)

    @classmethod
    def to_bytes(cls, instance: BaseStruct, *, show_progress = False) -> bytes:
        """
        Convert the struct object to bytes

        :param instance: The struct object to convert to bytes
        :param show_progress: When true, display a progress bar
        :return: bytes
        """
        length = len(instance._retrievers)

        bytes_ = [b""] * length
        compress_idx = length
        retriever_ls = instance._retrievers
        if show_progress:
            retriever_ls = alive_it(
                retriever_ls,
                dual_line = True,
                title = f"         Writing File",
                stats = False,
                finalize = lambda bar: bar.title(f"Finished Writing File")
            )

        for i, retriever in enumerate(retriever_ls):
            if show_progress:
                retriever_ls.text = f"            <- {retriever.p_name.title().replace('_', ' ')}"
            if retriever.remaining_compressed:
                compress_idx = i
            try:
                bytes_[i] = retriever.to_bytes(instance)
            except Exception:
                print(retriever.p_name, getattr(instance, retriever.p_name))
                raise

        compressed = b""
        if compress_idx != length:
            compressed = cls.compress(b"".join(bytes_[compress_idx:]))

        return b"".join(bytes_[:compress_idx]) + compressed

    def to_file(self, file_name: str):
        """
        Write the bytes of the struct object to a file

        :param file_name: The name of the file to write to
        """
        with open(file_name, "wb") as file:
            file.write(self.to_bytes(self, show_progress = True))

    def __repr__(self) -> str:
        repr_builder = StringIO()
        repr_builder.write(f"{self.__class__.__name__}(")
        for retriever in self._retrievers:
            if not retriever.supported(self.struct_ver):
                continue

            obj = getattr(self, retriever.p_name)
            if isinstance(obj, list):
                sub_obj_repr_str = (
                    "[\n    "
                    + ",\n    ".join(map(compose(indentify, repr), obj))
                    + ",\n]"
                )
            else:
                sub_obj_repr_str = f"{obj!r}"

            repr_builder.write(f"\n    {retriever.p_name} = {indentify(sub_obj_repr_str)},")
        repr_builder.write("\n)")
        return repr_builder.getvalue()

    # todo: write val <-> data (names) to file
    # todo: write hex (decompressed) to file
    # todo: str, eq, neq
    # todo: diff
    # todo: file/header/decompressed in both hex/val <-> data
    # todo: to_json
