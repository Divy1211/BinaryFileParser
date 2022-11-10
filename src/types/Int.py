from typing import Literal

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.types.ParserType import ParserType


class Int(ParserType):
    _byte_len = 4
    _signed = True

    @classmethod
    def from_generator(cls, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little") -> int:
        return cls.from_bytes(igen.get_bytes(cls._byte_len), byteorder)

    @classmethod
    def from_bytes(cls, bytes_: bytes, byteorder: Literal["big", "little"] = "little") -> int:
        return int.from_bytes(bytes_, byteorder, signed = cls._signed)

    @classmethod
    def to_bytes(cls, value: int, byteorder: Literal["big", "little"] = "little") -> bytes:
        return int.to_bytes(value, cls._byte_len, byteorder, signed = cls._signed)


class Int8(Int):
    _byte_len = 1

class Int16(Int):
    _byte_len = 2

class Int32(Int):
    _byte_len = 4

class Int64(Int):
    _byte_len = 8

class UInt8(Int):
    _byte_len = 1
    _signed = False

class UInt16(Int):
    _byte_len = 2
    _signed = False

class UInt32(Int):
    _byte_len = 4
    _signed = False

class UInt64(Int):
    _byte_len = 8
    _signed = False
