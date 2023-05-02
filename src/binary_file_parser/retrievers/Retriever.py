from __future__ import annotations


from copy import copy
from io import BytesIO
from typing import Type, Callable, TypeVar

from binary_file_parser.errors import VersionError
from binary_file_parser.types import ByteStream
from binary_file_parser.types import Parseable

from binary_file_parser.retrievers.MapValidate import MapValidate
from binary_file_parser.retrievers.base_struct import BaseStruct
from binary_file_parser.types.RefList import RefList
from binary_file_parser.utils import Version

T = TypeVar("T")
RetrieverSub = TypeVar("RetrieverSub", bound = "Retriever")
BaseStructSub = TypeVar("BaseStructSub", bound = BaseStruct)


class Retriever(MapValidate):
    """
    Represents a binary object in a struct and the restrictions/dependencies associated with it
    """
    __slots__ = "atype", "dtype", "min_ver", "max_ver", "default", "_repeat", "remaining_compressed", "on_read", "on_write"

    def __init__(
        self,
        dtype: Parseable | Type[Parseable],
        min_ver: Version = Version((-1,)),
        max_ver: Version = Version((1000,)),
        *,
        default: T = None,
        repeat: int = 1,
        atype: Type[RefList] = RefList,
        remaining_compressed: bool = False,
        on_read: list[Callable[[RetrieverSub, BaseStructSub], None]] | None = None,  # todo: add implementations for common on_x operations
        on_write: list[Callable[[RetrieverSub, BaseStructSub], None]] | None = None,
        mappers: list[Callable[[RetrieverSub, BaseStructSub, T], T]] | None = None,
        validators: list[Callable[[RetrieverSub, BaseStructSub, T], tuple[bool, str]]] | None = None,
        on_get: list[Callable[[RetrieverSub, BaseStructSub], None]] | None = None,
        on_set: list[Callable[[RetrieverSub, BaseStructSub], None]] | None = None,
    ):
        # todo: default factories and deep copies
        """
        :param dtype: The type of the value to read
        :param min_ver:
            The minimum struct version which supports this retriever property. If the version of the struct being read
            is less than min_ver, reading this retriever property is skipped and a version error is raised if an attempt
            to access or assign it is made
        :param max_ver:
            The maximum struct version which supports this retriever property. If the version of the struct being read
            is greater than max_ver, reading this retriever property is skipped and a version error is raised if an
            attempt to access or assign it is made
        :param default: A default value for this retriever property
        :param repeat:
            The number of times this value is repeated. Possible values for this parameter include:
                -1: skips reading value entirely, set property to None
                0: skips reading value entirely, set property to []
                1: single value is read and assigned to the property
                >1: a list of values is read and assigned to the property
        :param atype:
            The array like type to use when constructing lists for retrievers with dynamic repeats
        :param remaining_compressed:
            If set to true, the decompress/compress methods are used on the remaining bytes before reading/writing the
            remaining retriever properties
        :param on_read:
            A list of functions that are called when this retriever property is read from bytes for the first time
        :param on_write: A list of functions that are called when this retriever property is written to bytes
        :param mappers: A list of functions that can mutate the value that is assigned to this retriever property
        :param validators:
            A list of functions that can validate the value that is assigned to this retriever property.
            A ValueError is raised if validation from any one of these functions fails
        :param on_get: A list of functions that are called when this retriever property is accessed
        :param on_set: A list of functions that are called when this retriever property is set
        """
        super().__init__(mappers, validators, on_get, on_set)
        self.dtype = dtype
        self.atype = atype
        self.min_ver = min_ver
        self.max_ver = max_ver
        self.default = default
        self._repeat = repeat
        self.remaining_compressed = remaining_compressed
        self.on_read = on_read or []
        self.on_write = on_write or []

        # if self._repeat == 1:
        #     self.validators.append(lambda retriever, instance, x: dtype.is_valid(x))
        # else:
        #     self.validators.append(lambda retriever, instance, iterable: all(map(dtype.is_valid, iterable)))

    def supported(self, ver: Version) -> bool:
        """
        Determine if this retriever property is supported in the specified version

        :param ver: The version to check for support in
        :return: true if supported else false
        """
        return self.min_ver <= ver <= self.max_ver

    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None:
        super().__set_name__(owner, name)
        owner._add_retriever(self)

    def __set__(self, instance: BaseStruct, value: T) -> None:
        if not self.supported(instance.struct_ver):
            raise VersionError(
                f"{self.p_name!r} is not supported in your scenario version {instance.struct_ver}"
            )

        # def set_parent(obj):
        #     if isinstance(obj, BaseStruct):
        #         obj.parent = instance
        #     elif isinstance(obj, list):
        #         for sub_obj in obj:
        #             set_parent(sub_obj)
        #
        # set_parent(value)
        super().__set__(instance, value)

    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]) -> Retriever | T:
        if instance is None:
            return self

        if not self.supported(instance.struct_ver):
            raise VersionError(
                f"{self.p_name!r} is not supported in your struct version {instance.struct_ver}"
            )
        try:
            return super().__get__(instance, owner)
        except AttributeError:
            if self.default is None:
                raise ValueError(f"No default value specified for retriever {self.p_name!r}")
            return self.from_default(instance)

    @property
    def r_name(self) -> str:
        return f"_repeat_{self.p_name}"

    def set_repeat(self, instance: BaseStruct, repeat: int) -> None:
        """
        Set the repeat value of a retriever property for a provided struct object.

        :param instance: The struct object to set the repeat value of this retriever property for
        :param repeat: The repeat value to set
        """
        setattr(instance, self.r_name, repeat)

    def repeat(self, instance: BaseStruct) -> int:
        """
        Get the repeat value of a retriever property for a provided struct object

        :param instance: The struct object to get the repeat value of this retriever property from
        :return: The repeat value
        """
        if (repeat := getattr(instance, self.r_name, None)) is not None:
            return repeat
        return self._repeat

    def from_default(self, instance: BaseStruct):
        """
        Initialise this retriever property from its default value

        :param instance: The struct object to initialise the retriever property for
        """
        repeat = self.repeat(instance)
        if repeat == -1:
            return None

        val = self.default
        if self.dtype.is_struct:
            val = (
                val.__class__(instance.struct_ver, instance) if repeat == 1
                else [val.__class__(instance.struct_ver, instance) for _ in range(repeat)]
            )
        elif isinstance(val, list): # todo: this should probably be a deepcopy
            val = copy(val) if repeat == 1 else [copy(val) for _ in range(repeat)]
        elif repeat != 1: # todo: this should probably be a deepcopy too
            val = [copy(val) for _ in range(repeat)]

        if isinstance(val, list):
            val = self.atype(val, instance.struct_ver, instance)

        return val

    def from_stream(self, instance: BaseStruct, stream: ByteStream) -> None:
        """
        Initialise this retriever property from a stream

        :param instance: The struct object to initialise the retriever property for
        :param stream: The stream to initialise the retriever property from
        """
        if not self.supported(instance.struct_ver):
            return

        repeat = self.repeat(instance)
        if repeat == -1:
            setattr(instance, self.p_name, None)
            return

        def getobj():
            if self.dtype.is_struct:
                self.dtype: BaseStruct  # type: ignore
                return self.dtype.from_stream(stream, struct_ver = instance.struct_ver, parent = instance)
            return self.dtype.from_stream(stream, struct_ver = instance.struct_ver)

        is_not_dynamic_repeat = not hasattr(instance, self.r_name)
        if repeat == 1 and is_not_dynamic_repeat:
            setattr(instance, self.p_name, getobj())
            return

        ls: list = [None] * repeat
        for i in range(repeat):
            ls[i] = getobj()
        setattr(instance, self.p_name, self.atype(ls, instance.struct_ver, instance))

        for func in self.on_read:
            func(self, instance)

    def to_bytes(self, instance: BaseStruct) -> bytes:
        """
        Convert this retriever property to bytes

        :param instance: The struct object to convert the retriever property from
        :return: The bytes of the retriever property
        """
        if not self.supported(instance.struct_ver):
            return b""

        repeat = self.repeat(instance)
        if repeat == -1:
            return b""

        for func in self.on_write:
            func(self, instance)

        is_not_dynamic_repeat = not hasattr(instance, self.r_name)
        if repeat == 1 and is_not_dynamic_repeat:
            return self.dtype.to_bytes(getattr(instance, self.p_name))

        ls: list = getattr(instance, self.p_name)
        if len(ls) != repeat:
            raise ValueError(f"length of {self.p_name!r} is not the same as {repeat = }")

        bytes_ = BytesIO()
        for value in getattr(instance, self.p_name):
            bytes_.write(self.dtype.to_bytes(value))

        return bytes_.getvalue()
