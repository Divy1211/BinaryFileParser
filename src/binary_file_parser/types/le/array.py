from __future__ import annotations

import struct
from typing import Type, TYPE_CHECKING

from binary_file_parser.types.parseable import Parseable
from binary_file_parser.types.version import Version
from binary_file_parser.types.byte_stream import ByteStream

if TYPE_CHECKING:
    ParseableType = Type[Parseable] | Parseable

class BaseArray(Parseable):
    __slots__ = ("dtype", "struct_symbol", "length")

    def __init__(self, size: int, dtype: ParseableType, struct_symbol: str):
        super().__init__(size)
        self.dtype = dtype
        self.struct_symbol = struct_symbol
        self.length = -1

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> list:
        ls = [None] * self.length
        for i in range(self.length):
            ls[i] = self.dtype._from_stream(stream, struct_ver = struct_ver)
        return ls

    def _from_bytes(self, bytes_: bytes, *, struct_ver: Version = Version((0,))) -> list:
        return self._from_stream(ByteStream.from_bytes(bytes_), struct_ver = struct_ver)

    def _to_bytes(self, value: list) -> bytes:
        ls = [b""]*self.length
        for i, val in enumerate(value):
            ls[i] = self.dtype._to_bytes(val)
        return b"".join(ls)


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

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> list[list]:
        num_arrays = self.num_arrays
        if num_arrays == -1:
            num_arrays = struct.unpack(self.struct_symbol, stream.get(self._size))[0]

        lengths: list[int] = [struct.unpack(self.struct_symbol, stream.get(self._size))[0] for _ in range(num_arrays)]
        ls: list[list] = [[] for _ in range(num_arrays)]

        for i, length in enumerate(lengths):
            self.length = length
            ls[i] = super()._from_stream(stream, struct_ver = struct_ver)

        return ls

    def _to_bytes(self, value: list[list]) -> bytes:
        if self.num_arrays != -1 and len(value) != self.num_arrays:
            raise TypeError(f"Expected {self.num_arrays} StackedArrays, found {len(value)}")

        length_bytes = b""
        num_arrays = self.num_arrays
        if num_arrays == -1:
            num_arrays = len(value)
            length_bytes = struct.pack(self.struct_symbol, num_arrays)

        bytes_: list[bytes] = [b""]*(2*num_arrays)
        lengths = [len(ls) for ls in value]
        for i, length in enumerate(lengths):
            self.length = length
            bytes_[i] = struct.pack(self.struct_symbol, length)
            bytes_[num_arrays+i] = super()._to_bytes(value[i])

        return length_bytes+b"".join(bytes_)


class StackedArray8s(StackedArrays):
    """
    Represents a 2D array where the number of rows is indicated by a ``uint8``, followed by that many ``uint8s``,
    indicating the length of each row, followed by that many elements for each row.

    >>> StackedArray8s[int32]
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
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType | tuple[ParseableType, int]) -> StackedArrays:
        if isinstance(item, tuple):
            return cls(item[1], item[0], '<Q', item[1])
        return cls(8, item, '<Q')
