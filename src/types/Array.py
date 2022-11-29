from __future__ import annotations

import struct
from typing import Type

from src.types.ByteStream import ByteStream
from src.types.Parseable import Parseable

ParseableType = Type[Parseable] | Parseable

class BaseArray(Parseable):
    __slots__ = ("dtype", "struct_symbol", "length")

    def __init__(self, size: int, dtype: ParseableType, struct_symbol: str):
        super().__init__(size)
        self.dtype = dtype
        self.struct_symbol = struct_symbol
        self.length = -1

    def from_stream(self, stream: ByteStream, *, struct_version: tuple[int, ...] = (0,)) -> list:
        ls = [None] * self.length
        for i in range(self.length):
            ls[i] = self.dtype.from_stream(stream, struct_version = struct_version)
        return ls

    def from_bytes(self, bytes_: bytes, *, struct_version: tuple[int, ...] = (0,)) -> list:
        return self.from_stream(ByteStream.from_bytes(bytes_), struct_version = struct_version)

    def to_bytes(self, value: list) -> bytes:
        ls = [b""]*self.length
        for i, val in enumerate(value):
            ls[i] = self.dtype.to_bytes(val)
        return b"".join(ls)


class Array(BaseArray):
    __slots__ = ()

    def from_stream(self, stream: ByteStream, *, struct_version: tuple[int, ...] = (0,)) -> list:
        self.length = struct.unpack(self.struct_symbol, stream.get(self.size))[0]
        return super().from_stream(stream, struct_version = struct_version)

    def to_bytes(self, value: list) -> bytes:
        self.length = len(value)
        length_bytes = struct.pack(self.struct_symbol, self.length)
        return length_bytes+super().to_bytes(value)

class Array8(Array):
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType) -> Array8:
        return cls(1, item, '<B')

class Array16(Array):
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType) -> Array16:
        return cls(2, item, '<H')

class Array32(Array):
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType) -> Array32:
        return cls(4, item, '<I')

class Array64(Array):
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType) -> Array64:
        return cls(8, item, '<Q')


class FixedLenArray(BaseArray):
    __slots__ = ()

    def __init__(self, size: int, dtype: ParseableType, struct_symbol: str, length: int):
        super().__init__(size, dtype, struct_symbol)
        self.length = length

    def is_valid(self, value: list) -> tuple[bool, str]:
        if len(value) == self.length:
            return True, ""
        return False, f"%s must have a fixed length of {value}"

    def to_bytes(self, value: list) -> bytes:
        valid, msg = self.is_valid(value)
        if not valid:
            raise TypeError(msg)
        return super().to_bytes(value)

    def __class_getitem__(cls, item: tuple[ParseableType, int]) -> FixedLenArray:
        return cls(4, item[0], '<I', item[1])


class StackedArrays(BaseArray):
    __slots__ = "num_arrays"

    def __init__(self, size: int, dtype: ParseableType, struct_symbol: str, num_arrays: int = -1):
        super().__init__(size, dtype, struct_symbol)
        self.num_arrays = num_arrays

    def is_valid(self, value: list[list]) -> tuple[bool, str]:
        if self.num_arrays == -1:
            return True, ""

        num_arrays = len(value)
        if num_arrays == self.num_arrays:
            return True, ""
        return False, f"%s expected {self.num_arrays} but found {num_arrays} stacked arrays"

    def from_stream(self, stream: ByteStream, *, struct_version: tuple[int, ...] = (0,)) -> list[list]:
        num_arrays = self.num_arrays
        if num_arrays == -1:
            num_arrays = struct.unpack(self.struct_symbol, stream.get(self.size))[0]

        lengths: list[int] = [struct.unpack(self.struct_symbol, stream.get(self.size))[0] for _ in range(num_arrays)]
        ls: list[list] = [[] for _ in range(num_arrays)]

        for i, length in enumerate(lengths):
            self.length = length
            ls[i] = super().from_stream(stream, struct_version = struct_version)

        return ls

    def to_bytes(self, value: list[list]) -> bytes:
        valid, msg = self.is_valid(value)
        if not valid:
            raise TypeError(msg)

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
            bytes_[num_arrays+i] = super().to_bytes(value[i])

        return length_bytes+b"".join(bytes_)


class StackedArray8s(StackedArrays):
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType | tuple[ParseableType, int]) -> StackedArrays:
        if isinstance(item, tuple):
            return cls(4, item[0], '<B', item[1])
        return cls(4, item, '<B')


class StackedArray16s(StackedArrays):
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType | tuple[ParseableType, int]) -> StackedArrays:
        if isinstance(item, tuple):
            return cls(4, item[0], '<H', item[1])
        return cls(4, item, '<H')


class StackedArray32s(StackedArrays):
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType | tuple[ParseableType, int]) -> StackedArrays:
        if isinstance(item, tuple):
            return cls(4, item[0], '<I', item[1])
        return cls(4, item, '<I')


class StackedArray64s(StackedArrays):
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType | tuple[ParseableType, int]) -> StackedArrays:
        if isinstance(item, tuple):
            return cls(4, item[0], '<Q', item[1])
        return cls(4, item, '<Q')
