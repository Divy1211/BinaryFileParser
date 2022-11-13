from abc import ABC
from typing import Literal, Sized, Iterable, Callable

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.types.ParserType import ParserType


class BaseStr(ParserType, ABC):
    @classmethod
    def from_bytes(cls, bytes_: bytes, byteorder: Literal["big", "little"] = "little") -> str:
        try:
            return bytes_.decode("utf-8")
        except UnicodeDecodeError:
            return bytes_.decode("latin-1")

    @classmethod
    def to_bytes(cls, value: str, byteorder: Literal["big", "little"] = "little") -> bytes:
        try:
            bytes_ = value.encode("utf-8")
        except UnicodeEncodeError:
            bytes_ = value.encode("latin-1")

        return bytes_


class CStr(BaseStr):
    @classmethod
    def from_generator(cls, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little") -> str:
        bytes_ = b""
        while (byte := igen.get_bytes(1)) != b"\x00":
            bytes_ += byte
        return cls.from_bytes(bytes_)


class Str(BaseStr):
    _len_len = 4

    @classmethod
    def from_generator(cls, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little") -> str:
        length = int.from_bytes(igen.get_bytes(cls._len_len), "little", signed = False)
        return cls.from_bytes(igen.get_bytes(length))

    @classmethod
    def to_bytes(cls, value: str, byteorder: Literal["big", "little"] = "little") -> bytes:
        length = int.to_bytes(len(value), length = cls._len_len, byteorder = "little", signed = False)
        return length+super().to_bytes(value)

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

    @classmethod
    def from_generator(cls, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little") -> str:
        length = int.from_bytes(igen.get_bytes(cls._len_len), "little", signed = False)
        return cls.from_bytes(igen.get_bytes(length)[:-1])

    @classmethod
    def to_bytes(cls, value: str, byteorder: Literal["big", "little"] = "little") -> bytes:
        length = int.to_bytes(len(value)+1, length = cls._len_len, byteorder = "little", signed = False)
        return length+super().to_bytes(value)+b"\x00"

class NullTermStr8(NullTermStr):
    _len_len = 1

class NullTermStr16(NullTermStr):
    _len_len = 2

class NullTermStr32(NullTermStr):
    _len_len = 4

class NullTermStr64(NullTermStr):
    _len_len = 8

class FixedLenStr(BaseStr):
    def __init__(self, length: int):
        self.length = length

    def from_generator(self, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little") -> str:
        return self.from_bytes(igen.get_bytes(self.length))


def chk_len(length: int, iter_: Sized) -> tuple[bool, str]:
    valid = len(iter_) == length
    msg = ""
    if not valid:
        msg = f"%s must have a fixed length of {length}"
    return valid, msg

def each_len(cmp: Callable[[int, int], bool], length: int, iter_: Iterable[Sized]) -> tuple[bool, str]:
    for sizable in iter_:
        if not cmp(len(sizable), length):
            break
    else:
        return True, ""
    return False, f"each element of %s must have a fixed length {cmp.__name__} {length}"
