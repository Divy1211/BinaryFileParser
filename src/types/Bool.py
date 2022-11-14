from typing import Literal

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.types.ParserType import ParserType


class Bool(ParserType):
    _byte_len = 1

    @classmethod
    def from_generator(cls, igen: IncrementalGenerator, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,)) -> bool:
        return cls.from_bytes(igen.get_bytes(cls._byte_len), byteorder = byteorder)

    @classmethod
    def from_bytes(cls, bytes_: bytes, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,)) -> bool:
        return bool(int.from_bytes(bytes_, byteorder, signed = False))

    @classmethod
    def to_bytes(cls, value: bool, *, byteorder: Literal["big", "little"] = "little") -> bytes:
        return int.to_bytes(int(value), cls._byte_len, byteorder, signed = False)


class Bool8(Bool):
    _byte_len = 1

class Bool16(Bool):
    _byte_len = 2

class Bool32(Bool):
    _byte_len = 4

class Bool64(Bool):
    _byte_len = 8
