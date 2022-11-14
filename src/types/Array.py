from __future__ import annotations

from typing import Literal, Type

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.types.ParserType import ParserType

class BaseArray(ParserType):

    def __init__(self, cls_or_obj: Type[ParserType] | ParserType):
        self.cls_or_obj = cls_or_obj
        self.length = -1

    def from_generator(self, igen: IncrementalGenerator, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,)) -> list:
        ls = [None]*self.length
        for i in range(self.length):
            ls[i] = self.cls_or_obj.from_generator(igen, struct_version = struct_version)
        return ls

    def from_bytes(self, bytes_: bytes, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,)) -> list:
        return self.from_generator(IncrementalGenerator.from_bytes(bytes_), struct_version = struct_version)

    def to_bytes(self, value: list, byteorder: Literal["big", "little"] = "little") -> bytes:
        ls = [b""]*self.length

        for i, val in enumerate(value):
            ls[i] = self.cls_or_obj.to_bytes(val)

        return b"".join(ls)


class Array(BaseArray):
    def from_generator(self, igen: IncrementalGenerator, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,)) -> list:
        self.length = int.from_bytes(igen.get_bytes(4), "little", signed = False)
        return super().from_generator(igen, byteorder = byteorder, struct_version = struct_version)

    def from_bytes(self, bytes_: bytes, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,)) -> list:
        return self.from_generator(IncrementalGenerator.from_bytes(bytes_), struct_version = struct_version)

    def to_bytes(self, value: list, byteorder: Literal["big", "little"] = "little") -> bytes:
        self.length = len(value)
        length_bytes = int.to_bytes(self.length, length = 4, byteorder = "little", signed = False)
        return length_bytes+super().to_bytes(value, byteorder = byteorder)

    def __class_getitem__(cls, item: Type[ParserType] | ParserType) -> Array:
        return cls(item)


class FixedLenArray(BaseArray):

    def is_valid(self, value: list) -> tuple[bool, str]:
        if len(value) == self.length:
            return True, ""
        return False, f"%s must have a fixed length of {value}"

    def __init__(self, cls_or_obj: Type[ParserType] | ParserType, length: int,):
        super().__init__(cls_or_obj)
        self.length = length

    def __class_getitem__(cls, item: tuple[int, Type[ParserType] | ParserType]) -> FixedLenArray:
        return cls(item[1], item[0])
