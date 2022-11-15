from __future__ import annotations

from typing import Literal

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.types.ParserType import ParserType, ParserTypeObjCls

class BaseArray(ParserType):
    def __init__(self, cls_or_obj: ParserTypeObjCls):
        self.cls_or_obj = cls_or_obj
        self.length = -1

    def from_generator(self, igen: IncrementalGenerator, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,)) -> list:
        ls = [None]*self.length
        for i in range(self.length):
            ls[i] = self.cls_or_obj.from_generator(igen, struct_version = struct_version)
        return ls

    def from_bytes(self, bytes_: bytes, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,)) -> list:
        return self.from_generator(IncrementalGenerator.from_bytes(bytes_), struct_version = struct_version)

    def to_bytes(self, value: list, *, byteorder: Literal["big", "little"] = "little") -> bytes:
        ls = [b""]*self.length
        for i, val in enumerate(value):
            ls[i] = self.cls_or_obj.to_bytes(val)
        return b"".join(ls)


class Array(BaseArray):
    _len_len = 4

    def from_generator(self, igen: IncrementalGenerator, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,)) -> list:
        self.length = int.from_bytes(igen.get_bytes(self._len_len), "little", signed = False)
        return super().from_generator(igen, byteorder = byteorder, struct_version = struct_version)

    def to_bytes(self, value: list, *, byteorder: Literal["big", "little"] = "little") -> bytes:
        self.length = len(value)
        length_bytes = int.to_bytes(self.length, length = self._len_len, byteorder = "little", signed = False)
        return length_bytes+super().to_bytes(value, byteorder = byteorder)

    def __class_getitem__(cls, item: ParserTypeObjCls) -> Array:
        return cls(item)

class Array32(Array):
    _len_len = 4

class Array16(Array):
    _len_len = 2


class FixedLenArray(BaseArray):
    def is_valid(self, value: list) -> tuple[bool, str]:
        if len(value) == self.length:
            return True, ""
        return False, f"%s must have a fixed length of {value}"

    def __init__(self, cls_or_obj: ParserTypeObjCls, length: int,):
        super().__init__(cls_or_obj)
        self.length = length

    def to_bytes(self, value: list, *, byteorder: Literal["big", "little"] = "little") -> bytes:
        valid, msg = self.is_valid(value)
        if not valid:
            raise TypeError(msg)
        return super().to_bytes(value, byteorder=byteorder)

    def __class_getitem__(cls, item: tuple[int, ParserTypeObjCls]) -> FixedLenArray:
        return cls(item[1], item[0])


class StackedArrays(BaseArray):
    _len_len = 4

    def __init__(self, cls_or_obj: ParserTypeObjCls, num_arrays: int):
        super().__init__(cls_or_obj)
        self.num_arrays = num_arrays

    def is_valid(self, value: list[list]) -> tuple[bool, str]:
        if self.num_arrays == -1:
            return True, ""

        num_arrays = len(value)
        if num_arrays == self.num_arrays:
            return True, ""
        return False, f"%s expected {self.num_arrays} but found {num_arrays} stacked arrays"

    def from_generator(self, igen: IncrementalGenerator, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,)) -> list[list]:
        num_arrays = self.num_arrays
        if num_arrays == -1:
            num_arrays = int.from_bytes(igen.get_bytes(self._len_len), "little", signed = False)

        lengths: list[int] = [int.from_bytes(igen.get_bytes(self._len_len), "little", signed = False) for _ in range(num_arrays)]
        ls: list[list] = [[] for _ in range(num_arrays)]

        for i, length in enumerate(lengths):
            self.length = length
            ls[i] = super().from_generator(igen, byteorder = byteorder, struct_version = struct_version)

        return ls

    def to_bytes(self, value: list[list], *, byteorder: Literal["big", "little"] = "little") -> bytes:
        valid, msg = self.is_valid(value)
        if not valid:
            raise TypeError(msg)

        length_bytes = b""
        num_arrays = self.num_arrays
        if num_arrays == -1:
            num_arrays = len(value)
            length_bytes = int.to_bytes(num_arrays, length = self._len_len, byteorder = "little", signed = False)

        bytes_: list[bytes] = [b""]*(2*num_arrays)
        lengths = [len(ls) for ls in value]
        for i, length in enumerate(lengths):
            self.length = length
            bytes_[i] = int.to_bytes(length, length = 4, byteorder = "little", signed = False)
            bytes_[num_arrays+i] = super().to_bytes(value[i], byteorder = byteorder)

        return length_bytes+b"".join(bytes_)

    def __class_getitem__(cls, item: ParserTypeObjCls | tuple[ParserTypeObjCls, int]) -> StackedArrays:
        if isinstance(item, tuple):
            return cls(item[0], item[1])
        return cls(item, -1)

class StackedArray32s(StackedArrays):
    _len_len = 4
