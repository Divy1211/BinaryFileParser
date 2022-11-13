import struct
from typing import Literal

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.types.ParserType import ParserType


class Float(ParserType):
    _byte_len = 4
    _struct_symbol = 'f'

    @classmethod
    def from_generator(cls, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> float:
        return cls.from_bytes(igen.get_bytes(cls._byte_len), byteorder)

    @classmethod
    def from_bytes(cls, bytes_: bytes, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> float:
        return struct.unpack(cls._struct_symbol, bytes_)[0]

    @classmethod
    def to_bytes(cls, value: float, byteorder: Literal["big", "little"] = "little") -> bytes:
        return struct.pack(cls._struct_symbol, value)


class Float16(Float):
    _byte_len = 2
    _struct_symbol = 'e'

class Float32(Float):
    _byte_len = 4
    _struct_symbol = 'f'

class Float64(Float):
    _byte_len = 8
    _struct_symbol = 'd'
