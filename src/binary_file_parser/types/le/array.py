from __future__ import annotations

import struct
from contextlib import suppress
from typing import Type, TYPE_CHECKING

from binary_file_parser.errors import VersionError
from binary_file_parser.types.le.option import Option
from binary_file_parser.types.parseable import Parseable
from binary_file_parser.types.version import Version
from binary_file_parser.types.byte_stream import ByteStream

if TYPE_CHECKING:
    from binary_file_parser.types.base_struct import BaseStruct
    ParseableType = Type[Parseable] | Parseable

class BaseArray(Parseable):
    __slots__ = ("dtype", "struct_symbol", "length")

    def __init__(self, size: int, dtype: ParseableType, struct_symbol: str):
        super().__init__(size)
        self.dtype = dtype
        self.struct_symbol = struct_symbol
        self.length = -1

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> list:
        return [
            self.dtype._from_stream(stream, struct_ver = struct_ver)
            for _ in range(self.length)
        ]

    def _from_bytes(self, bytes_: bytes, *, struct_ver: Version = Version((0,))) -> list:
        return self._from_stream(ByteStream.from_bytes(bytes_), struct_ver = struct_ver)

    def _to_bytes(self, value: list) -> bytes:
        return b"".join(
            self.dtype._to_bytes(val)
            for val in value
        )

class Array(BaseArray):
    __slots__ = ()

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> list:
        self.length = struct.unpack(self.struct_symbol, stream.get(self._size))[0]
        return super()._from_stream(stream, struct_ver = struct_ver)

    def _to_bytes(self, value: list) -> bytes:
        self.length = len(value)
        length_bytes = struct.pack(self.struct_symbol, self.length)
        return length_bytes+super()._to_bytes(value)

class Array8(Array):
    """
    Represents an array whose length is indicated by a uint8 followed by that many elements of the indicated type.
    Usage:

    >>> Array8[int32]
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType) -> Array8:
        return cls(1, item, '<B')

class Array16(Array):
    """
    Represents an array whose length is indicated by a ``uint16`` followed by that many elements of the indicated type.
    Usage:

    >>> Array16[int32]
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType) -> Array16:
        return cls(2, item, '<H')

class Array32(Array):
    """
    Represents an array whose length is indicated by a ``uint32`` followed by that many elements of the indicated type.
    Usage:

    >>> Array32[int32]
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType) -> Array32:
        return cls(4, item, '<I')

class Array64(Array):
    """
    Represents an array whose length is indicated by a ``uint64`` followed by that many elements of the indicated type.
    Usage:

    >>> Array64[int32]
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType) -> Array64:
        return cls(8, item, '<Q')


class FixedLenArray(BaseArray):
    """
    Represents an array whose length is known. This length is not read from/written to bytes
    Usage:

    >>> FixedLenArray[int32, 5]
    """
    __slots__ = ()

    def __init__(self, size: int, dtype: ParseableType, struct_symbol: str, length: int):
        super().__init__(size, dtype, struct_symbol)
        self.length = length

    def _to_bytes(self, value: list) -> bytes:
        if len(value) != self.length:
            raise TypeError(f"Expected FixedLenArray[{self.length}], found array with length: {len(value)}")
        return super()._to_bytes(value)

    def __class_getitem__(cls, item: tuple[ParseableType, int]) -> FixedLenArray:
        return cls(item[1], item[0], '<I', item[1])


class StackedArrays(BaseArray):
    __slots__ = "num_arrays"

    def __init__(self, size: int, dtype: ParseableType, struct_symbol: str, num_arrays: int = -1):
        super().__init__(size, dtype, struct_symbol)
        self.num_arrays = num_arrays

    def _read_with_length(self, stream: ByteStream, *, struct_ver: Version = Version((0,)), length: int) -> list:
        self.length = length
        return super()._from_stream(stream, struct_ver = struct_ver)

    def _write_with_length(self, val: list, length: int) -> bytes:
        self.length = length
        return super()._to_bytes(val)

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> list[list]:
        num_arrays = self.num_arrays
        if num_arrays == -1:
            num_arrays = struct.unpack(self.struct_symbol, stream.get(self._size))[0]

        lengths: list[int] = [
            struct.unpack(self.struct_symbol, stream.get(self._size))[0]
            for _ in range(num_arrays)
        ]

        return [
            self._read_with_length(stream, struct_ver = struct_ver, length = length)
            for length in lengths
        ]

    def _to_bytes(self, value: list[list]) -> bytes:
        if self.num_arrays != -1 and len(value) != self.num_arrays:
            raise TypeError(f"Expected {self.num_arrays} StackedArrays, found {len(value)}")

        length_bytes = b""
        num_arrays = self.num_arrays
        if num_arrays == -1:
            num_arrays = len(value)
            length_bytes = struct.pack(self.struct_symbol, num_arrays)

        len_bytes = (
            struct.pack(self.struct_symbol, len(ls))
            for ls in value
        )

        ls_bytes = (
            self._write_with_length(ls, len(ls))
            for ls in value
        )

        return length_bytes + b"".join(
            *len_bytes,
            *ls_bytes
        )


class StackedArray8s(StackedArrays):
    """
    Represents a 2D array where the number of rows is indicated by a ``uint8``, followed by that many ``uint8s``,
    indicating the length of each row, followed by that many elements for each row.

    >>> StackedArray8s[int32]
    >>> StackedArray8s[int32, 4] # indicate the number of rows as fixed. This excludes it from being read from/written to bytes
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType | tuple[ParseableType, int]) -> StackedArrays:
        if isinstance(item, tuple):
            return cls(item[1], item[0], '<B', item[1])
        return cls(1, item, '<B')


class StackedArray16s(StackedArrays):
    """
    Represents a 2D array where the number of rows is indicated by a ``uint16``, followed by that many ``uint16s``,
    indicating the length of each row, followed by that many elements for each row.

    >>> StackedArray16s[int32]
    >>> StackedArray16s[int32, 4] # indicate the number of rows as fixed. This excludes it from being read from/written to bytes
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType | tuple[ParseableType, int]) -> StackedArrays:
        if isinstance(item, tuple):
            return cls(item[1], item[0], '<H', item[1])
        return cls(2, item, '<H')


class StackedArray32s(StackedArrays):
    """
    Represents a 2D array where the number of rows is indicated by a ``uint32``, followed by that many ``uint32s``,
    indicating the length of each row, followed by that many elements for each row.

    >>> StackedArray32s[int32]
    >>> StackedArray32s[int32, 4] # indicate the number of rows as fixed. This excludes it from being read from/written to bytes
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType | tuple[ParseableType, int]) -> StackedArrays:
        if isinstance(item, tuple):
            return cls(4, item[0], '<I', item[1])
        return cls(4, item, '<I')


class StackedArray64s(StackedArrays):
    """
    Represents a 2D array where the number of rows is indicated by a ``uint64``, followed by that many ``uint64s``,
    indicating the length of each row, followed by that many elements for each row.

    >>> StackedArray64s[int32]
    >>> StackedArray64s[int32, 4] # indicate the number of rows as fixed. This excludes it from being read from/written to bytes
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType | tuple[ParseableType, int]) -> StackedArrays:
        if isinstance(item, tuple):
            return cls(item[1], item[0], '<Q', item[1])
        return cls(8, item, '<Q')


class StackedAttrArray(BaseArray):
    def __init__(self, size: int, dtype: Type[BaseStruct | Option], struct_symbol: str, length: int = -1):
        super().__init__(size, dtype, struct_symbol)
        self.length = length
        self.stype = dtype

    def _read_opt(self, stream: ByteStream, struct_ver: Version, length: int) -> list:
        exists = struct.unpack(
            f"<{self.stype.struct_symbol[1:] * length}",
            stream.get(self.stype._size * length)
        )
        return [
            self.stype.dtype._from_stream(stream, struct_ver = struct_ver) if does_exist else None
            for does_exist in exists
        ]

    def _write_opt(self, value: list, length: int) -> bytes:
        exist_bytes = struct.pack(
            f"<{self.stype.struct_symbol[1:] * length}",
            *map(lambda x: x is not None,  value)
        )
        ls_bytes = (
            self.stype.dtype._to_bytes(ls)
            for ls in value
            if ls is not None
        )
        return b"".join(
            *exist_bytes,
            *ls_bytes
        )

    def _read_with_ver(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> BaseStruct:
        instance = self.stype(struct_ver, initialise_defaults = False)
        with suppress(VersionError):
            instance._get_version(stream, struct_ver)
        return instance

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> list:
        length = self.length
        if length == -1:
            length = struct.unpack(self.struct_symbol, stream.get(self._size))[0]

        if isinstance(self.stype, Option):
            return self._read_opt(stream, struct_ver, length)

        instances = [
            self._read_with_ver(stream, struct_ver = struct_ver)
            for _ in range(length)
        ]

        for retriever in self.stype._retrievers:
            for instance in instances:
                retriever.from_stream(instance, stream)

        return instances

    def _to_bytes(self, value: list) -> bytes:
        if self.length != -1 and len(value) != self.length:
            raise TypeError(f"Expected an array of length {self.length}, found array with length: {len(value)}")

        length_bytes = b""
        length = self.length
        if length == -1:
            length = len(value)
            length_bytes = struct.pack(self.struct_symbol, length)

        if isinstance(self.stype, Option):
            return length_bytes+self._write_opt(value, length)

        ls_bytes = (
            retriever.to_bytes(instance)
            for retriever in self.stype._retrievers
            for instance in value
        )

        return length_bytes+b"".join(ls_bytes)

class StackedAttrArray8(StackedAttrArray):
    """
    Represents an array of struct objects where the number of objects is indicated by a ``uint8``, followed by a list of
    that length for each attribute of the base struct

    >>> StackedAttrArray8[BaseStruct]
    >>> StackedAttrArray8[BaseStruct, 4] # indicate the number of objects as fixed. This excludes it from being read from/written to bytes
    """
    __slots__ = ()

    def __class_getitem__(cls, item: Type[BaseStruct | Option] | tuple[Type[BaseStruct | Option], int]) -> StackedAttrArray:
        if isinstance(item, tuple):
            return cls(item[1], item[0], '<B', item[1])
        return cls(1, item, '<B')


class StackedAttrArray16(StackedAttrArray):
    """
    Represents an array of struct objects where the number of objects is indicated by a ``uint16``, followed by a list of
    that length for each attribute of the base struct

    >>> StackedAttrArray16[BaseStruct]
    >>> StackedAttrArray16[BaseStruct, 4] # indicate the number of objects as fixed. This excludes it from being read from/written to bytes
    """
    __slots__ = ()

    def __class_getitem__(cls, item: Type[BaseStruct | Option] | tuple[Type[BaseStruct | Option], int]) -> StackedAttrArray:
        if isinstance(item, tuple):
            return cls(item[1], item[0], '<H', item[1])
        return cls(2, item, '<H')


class StackedAttrArray32(StackedAttrArray):
    """
    Represents an array of struct objects where the number of objects is indicated by a ``uint32``, followed by a list of
    that length for each attribute of the base struct

    >>> StackedAttrArray32[BaseStruct]
    >>> StackedAttrArray32[BaseStruct, 4] # indicate the number of objects as fixed. This excludes it from being read from/written to bytes
    """
    __slots__ = ()

    def __class_getitem__(cls, item: Type[BaseStruct | Option] | tuple[Type[BaseStruct | Option], int]) -> StackedAttrArray:
        if isinstance(item, tuple):
            return cls(4, item[0], '<I', item[1])
        return cls(4, item, '<I')


class StackedAttrArray64(StackedAttrArray):
    """
    Represents an array of struct objects where the number of objects is indicated by a ``uint64``, followed by a list of
    that length for each attribute of the base struct

    >>> StackedAttrArray64[BaseStruct]
    >>> StackedAttrArray64[BaseStruct, 4] # indicate the number of objects as fixed. This excludes it from being read from/written to bytes
    """
    __slots__ = ()

    def __class_getitem__(cls, item: Type[BaseStruct | Option] | tuple[Type[BaseStruct | Option], int]) -> StackedAttrArray:
        if isinstance(item, tuple):
            return cls(item[1], item[0], '<Q', item[1])
        return cls(8, item, '<Q')
