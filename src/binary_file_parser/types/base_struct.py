from __future__ import annotations

import copy
from contextlib import suppress
from io import StringIO
from typing import TYPE_CHECKING

from alive_progress import alive_it

from binary_file_parser.errors import CompressionError, ParsingError, VersionError
from binary_file_parser.types.parseable import Parseable
from binary_file_parser.types.byte_stream import ByteStream
# from binary_file_parser.types.ref_list import RefList
from binary_file_parser.types.version import Version
from binary_file_parser.utils import indentify

if TYPE_CHECKING:
    from binary_file_parser.retrievers import Retriever


class BaseStruct(Parseable):
    """
    Base class for defining a file format as a structure
    """
    __slots__ = "_struct_ver"

    _retrievers: list[Retriever] = []
    # _refs: list[RetrieverRef] = []
    # _combiners: list[RetrieverCombiner] = []

    @classmethod
    def _add_retriever(cls, retriever: Retriever):
        cls._retrievers.append(retriever)

    # @classmethod
    # def _add_ref(cls, ref: RetrieverRef):
    #     cls._refs.append(ref)
    #
    # @classmethod
    # def _add_combiner(cls, combiner: RetrieverCombiner):
    #     cls._combiners.append(combiner)

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
        # cls._refs, BaseStruct._refs = cls._refs.copy(), []
        # cls._combiners, BaseStruct._combiners = cls._combiners.copy(), []

    def __deepcopy__(self, memo):
        cls = self.__class__
        cloned_struct = cls.__new__(cls)
        memo[id(self)] = cloned_struct

        for attr in list(self.__slots__) + list(self.__dict__):
            value = copy.deepcopy(getattr(self, attr), memo)
            setattr(cloned_struct, attr, value)

        return cloned_struct

    @property
    def _struct(self):
        return self

    # @property
    # def root(self) -> BaseStruct:
    #     """
    #     The top level object of which this struct is a part of
    #     """
    #     child = self
    #     while child.parent is not None:
    #         child = self.parent
    #     return child

    def _on_struct_ver_change(self, new_struct_ver: Version):
        """
        This method is called when a struct's version is changed. Use this to implement any custom logic required to
        support a version change on a struct. The default method implements a trial and error algorithm where if an
        unsupported retriever after the version change used to have the same value as its default, then it is discarded
        without any errors, However, if a value other than the default was assigned to it, an error is raised

        :param new_struct_ver: The new struct version (the self.struct_ver attribute will automatically be updated)

        :raises VersionError: When the version change causes a non default valued retriever to become unsupported
        """
        pass

    @property
    def struct_ver(self) -> Version:
        return self._struct_ver

    # @struct_ver.setter
    # def struct_ver(self, value: Version):
    #     self._struct_ver = value

    # @property
    # def struct_ver(self):
    #     return self._struct_ver
    #
    # @struct_ver.setter
    # def struct_ver(self, new_struct_ver: Version):
    #     for retriever in self._retrievers:
    #         if (
    #             not retriever.supported(self.struct_ver)
    #             or not (retriever.dtype.is_struct or retriever.dtype.is_iterable)
    #             or retriever.is_self_versioned
    #         ):
    #             continue
    #         if (obj := getattr(self, retriever.p_name)) is not None:
    #             obj.struct_ver = new_struct_ver
    #     self.on_struct_ver_change(new_struct_ver)
    #     self._struct_ver = new_struct_ver
    #
    # @property
    # def parent(self):
    #     return self._parent
    #
    # @parent.setter
    # def parent(self, new_parent: BaseStruct):
    #     for retriever in self._retrievers:
    #         if (
    #             not retriever.supported(self.struct_ver)
    #             or not (retriever.dtype.is_struct or retriever.dtype.is_iterable)
    #         ):
    #             continue
    #         if (obj := getattr(self, retriever.p_name)) is not None:
    #             obj.parent = new_parent
    #     self._parent = new_parent

    @property
    def _retriever_name_value_map(self) -> dict[str]:
        map_ = {}
        for retriever in self._retrievers:
            if retriever.supported(self.struct_ver):
                map_[retriever.p_name] = getattr(self, retriever.p_name)
        return map_

    @classmethod
    def _get_version(cls, stream: ByteStream, struct_ver: Version = Version((0,))) -> Version:
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

    def _map(self) -> BaseStruct:
        """
        This method is called after a struct is read from bytes to allow any modifications post reading

        :return: A BaseStruct instance
        """
        return self

    @classmethod
    def _from_stream(
        cls, stream: ByteStream, *, struct_ver: Version = Version((0,)), strict: bool = False,
        show_progress: bool = False,
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
                stream = ByteStream.from_bytes(cls._decompress(stream.remaining()))
            retriever.from_stream(instance, stream)
            # instance._size += retriever.dtype._size

        file_len = len(stream.content)

        if stream.progress != file_len and strict:
            raise ParsingError(
                f"{file_len - stream.progress} bytes are left after parsing all retrievers successfully:\n"
                f"{stream.remaining()}"
            )

        return instance._map()

    @classmethod
    def _from_bytes(
        cls, bytes_: bytes, *, struct_ver: Version = Version((0,)), strict = False,
        show_progress: bool = False,
    ) -> BaseStruct:
        """
        Create a struct object from bytes

        :param bytes_: The bytes to create the struct object from
        :param struct_ver: The version of the structure to create. Overwritten if `get_version` is defined
        :param strict: Raise an error if struct parsing finishes successfully but there are unused bytes left over
        :param show_progress: When true, display a progress bar
        :return: An instance of a subtype of BaseStruct
        """
        stream = ByteStream.from_bytes(bytes_)
        return cls._from_stream(stream, struct_ver = struct_ver, strict = strict, show_progress = show_progress)

    @classmethod
    def _from_file(
        cls, file_name: str, *, file_version: Version = Version((0,)), strict = True,
        show_progress: bool = True,
    ) -> BaseStruct:
        """
        Create a struct object from file

        :param file_name: The path of the file to create the struct object from
        :param file_version: The version of the structure to create. Overwritten if `get_version` is defined
        :param strict: Raise an error if struct parsing finishes successfully but the stream has left over bytes
        :param show_progress: When true, display a progress bar
        :return: An instance of a subtype of BaseStruct
        """
        stream = ByteStream.from_file(file_name)
        return cls._from_stream(stream, struct_ver = file_version, strict = strict, show_progress = show_progress)

    @classmethod
    def _to_bytes(cls, instance: BaseStruct, *, show_progress = False) -> bytes:
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
                title = "         Writing File",
                stats = False,
                finalize = lambda bar: bar.title("Finished Writing File"),
            )

        for i, retriever in enumerate(retriever_ls):
            if show_progress:
                retriever_ls.text = f"            <- {retriever.p_name.title().replace('_', ' ')}"
            if retriever.remaining_compressed:
                compress_idx = i
            bytes_[i] = retriever.to_bytes(instance)

        compressed = b""
        if compress_idx != length:
            compressed = cls._compress(b"".join(bytes_[compress_idx:]))

        return b"".join(bytes_[:compress_idx]) + compressed

    def _to_file(self, file_name: str):
        """
        Write the bytes of the struct object to a file

        :param file_name: The name of the file to write to
        """
        with open(file_name, "wb") as file:
            file.write(self._to_bytes(self, show_progress = True))

    def _diff(self, other: BaseStruct) -> list[Retriever]:
        """
        Recursively builds a list of retrievers that have different values for the two objects.

        :param other: The object to diff with
        :return: list of retrievers that have different values for the two objects
        """
        if (
            not isinstance(other, BaseStruct)
            or type(self) is not type(other)
        ):
            raise TypeError(f"Cannot diff '{type(self)}' with an object of type '{type(other)}'")
        if self.struct_ver != other.struct_ver:
            raise VersionError(
                f"Cannot diff structs with different versions '{self.struct_ver}' and '{other.struct_ver}'"
            )

        diff_retrievers: list[Retriever] = []
        for retriever in self._retrievers:
            if not retriever.supported(self.struct_ver):
                continue
            if (val1 := getattr(self, retriever.p_name)) == (val2 := getattr(other, retriever.p_name)):
                continue
            if isinstance(val1, BaseStruct):
                diff_retrievers.extend(val1._diff(val2))
            else:
                diff_retrievers.append(retriever)
        return diff_retrievers

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
                    + ",\n    ".join(map(lambda x: indentify(repr(x)), obj))
                    + ",\n]"
                )
            else:
                sub_obj_repr_str = f"{obj!r}"

            repr_builder.write(f"\n    {retriever.p_name} = {indentify(sub_obj_repr_str)},")
        repr_builder.write("\n)")
        return repr_builder.getvalue()

    def __eq__(self, other: object) -> bool:
        if (
            not isinstance(other, BaseStruct)
            or type(self) is not type(other)
            or self.struct_ver != other.struct_ver
        ):
            return False

        for retriever in self._retrievers:
            if not retriever.supported(self.struct_ver):
                continue
            if getattr(self, retriever.p_name) != getattr(other, retriever.p_name):
                return False
        return True

    # todo: write val <-> data (names) to file
    # todo: write hex (decompressed) to file
    # todo: file/header/decompressed in both hex/val <-> data
    # todo: to_json
