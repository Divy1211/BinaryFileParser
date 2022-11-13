from typing import Any, Literal, Type

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.types.ParserType import ParserType


class Array(ParserType):
    __slots__ = "cls"

    def __init__(self, cls: Type[ParserType]):
        self.cls = cls

    def from_generator(self, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little") -> list:
        length = int.from_bytes(igen.get_bytes(4), "little", signed = False)
        ls = [None]*length
        for i in range(length):
            ls[i] = self.cls.from_generator(igen)
        return ls

    def from_bytes(self, bytes_: bytes, byteorder: Literal["big", "little"] = "little") -> list:
        return self.from_generator(IncrementalGenerator.from_bytes(bytes_))

    def to_bytes(self, value: list, byteorder: Literal["big", "little"] = "little") -> bytes:
        length = len(value)
        ls = [b""]*length

        for i, val in enumerate(value):
            ls[i] = self.cls.to_bytes(val)

        length_bytes = int.to_bytes(length, length = 4, byteorder = "little", signed = False)
        return length_bytes+b"".join(ls)
