from abc import ABC
from typing import Literal

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.types.ParserType import ParserType


class BaseStr(ParserType, ABC):
    @classmethod
    def from_bytes(cls, bytes_: bytes, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> str:
        try:
            return bytes_.decode("utf-8")
        except UnicodeDecodeError:
            return bytes_.decode("latin-1")

    # todo: pass plain bytes
    @classmethod
    def to_bytes(cls, value: str, byteorder: Literal["big", "little"] = "little") -> bytes:
        try:
            bytes_ = value.encode("utf-8")
        except UnicodeEncodeError:
            bytes_ = value.encode("latin-1")

        return bytes_


class CStr(BaseStr):
    @classmethod
    def from_generator(cls, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> str:
        bytes_ = b""
        while (byte := igen.get_bytes(1)) != b"\x00":
            bytes_ += byte
        return cls.from_bytes(bytes_)

    @classmethod
    def to_bytes(cls, value: str, byteorder: Literal["big", "little"] = "little") -> bytes:
        return super().to_bytes(value, byteorder)+b"\x00"

class Str(BaseStr):
    _len_len = 4

    @classmethod
    def from_generator(cls, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> str:
        length = int.from_bytes(igen.get_bytes(cls._len_len), "little", signed = False)
        return cls.from_bytes(igen.get_bytes(length))

    @classmethod
    def to_bytes(cls, value: str, byteorder: Literal["big", "little"] = "little") -> bytes:
        bytes_ = super().to_bytes(value)
        length = int.to_bytes(len(bytes_), length = cls._len_len, byteorder = "little", signed = False)
        return length+bytes_

class Str8(Str):
    _len_len = 1

class Str16(Str):
    _len_len = 2

class Str32(Str):
    _len_len = 4

class Str64(Str):
    _len_len = 8

class NullTermStr(BaseStr):
    _len_len = 4
    _no_null_when_empty = False

    @classmethod
    def from_generator(cls, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> str:
        length = int.from_bytes(igen.get_bytes(cls._len_len), "little", signed = False)
        if cls._no_null_when_empty and length == 0:
            return ""
        return cls.from_bytes(igen.get_bytes(length)[:-1])

    @classmethod
    def to_bytes(cls, value: str, byteorder: Literal["big", "little"] = "little") -> bytes:
        bytes_ = super().to_bytes(value)
        if not cls._no_null_when_empty or len(bytes_) != 0:
            bytes_ += b"\x00"
        length = int.to_bytes(len(bytes_), length = cls._len_len, byteorder = "little", signed = False)
        return length + bytes_


class NullTermStr8(NullTermStr):
    _len_len = 1

class NullTermStr16(NullTermStr):
    _len_len = 2

class NullTermStr32(NullTermStr):
    _len_len = 4

class NullTermStr64(NullTermStr):
    _len_len = 8

class NullTermNonEmptyStr8(NullTermStr):
    _len_len = 1
    _no_null_when_empty = True

class NullTermNonEmptyStr16(NullTermStr):
    _len_len = 2
    _no_null_when_empty = True

class NullTermNonEmptyStr32(NullTermStr):
    _len_len = 4
    _no_null_when_empty = True

class NullTermNonEmptyStr64(NullTermStr):
    _len_len = 8
    _no_null_when_empty = True

class FixedLenStr(BaseStr):
    __slots__ = "length",

    def is_valid(self, value: str) -> tuple[bool, str]:
        if len(value) == self.length:
            return True, ""
        return False, f"%s must have a fixed length of {value}"

    def __init__(self, length: int):
        self.length = length

    def from_generator(self, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> str:
        return self.from_bytes(igen.get_bytes(self.length))
