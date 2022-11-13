from typing import Literal, Type

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.types.ParserType import ParserType


class Array(ParserType):
    __slots__ = "cls",

    def __init__(self, cls: Type[ParserType]):
        self.cls = cls

    def from_generator(self, igen: IncrementalGenerator, *, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> list:
        length = int.from_bytes(igen.get_bytes(4), "little", signed = False)
        ls = [None]*length
        for i in range(length):
            ls[i] = self.cls.from_generator(igen, file_version = file_version)
        return ls

    def from_bytes(self, bytes_: bytes, *, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> list:
        return self.from_generator(IncrementalGenerator.from_bytes(bytes_), file_version = file_version)

    def to_bytes(self, value: list, byteorder: Literal["big", "little"] = "little") -> bytes:
        length = len(value)
        ls = [b""]*length

        for i, val in enumerate(value):
            ls[i] = self.cls.to_bytes(val)

        length_bytes = int.to_bytes(length, length = 4, byteorder = "little", signed = False)
        return length_bytes+b"".join(ls)


class FixedLenArray(Array):
    __slots__ = "length",

    def is_valid(self, value: list) -> tuple[bool, str]:
        if len(value) == self.length:
            return True, ""
        return False, f"%s must have a fixed length of {value}"

    def __init__(self, length: int, cls: Type[ParserType]):
        super().__init__(cls)
        self.length = length

    def from_generator(self, igen: IncrementalGenerator, *, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> list:
        ls = [None]*self.length
        for i in range(self.length):
            ls[i] = self.cls.from_generator(igen, file_version = file_version)
        return ls

    def from_bytes(self, bytes_: bytes, *, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> list:
        return self.from_generator(IncrementalGenerator.from_bytes(bytes_), file_version = file_version)

    def to_bytes(self, value: list, byteorder: Literal["big", "little"] = "little") -> bytes:
        ls = [b""]*self.length

        for i, val in enumerate(value):
            ls[i] = self.cls.to_bytes(val)

        return b"".join(ls)
