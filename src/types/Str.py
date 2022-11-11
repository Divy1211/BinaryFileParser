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


class Str(BaseStr):
    @classmethod
    def from_generator(cls, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little") -> str:
        length = int.from_bytes(igen.get_bytes(4), "little", signed = False)
        return cls.from_bytes(igen.get_bytes(length))

    @classmethod
    def to_bytes(cls, value: str, byteorder: Literal["big", "little"] = "little") -> bytes:
        length = int.to_bytes(len(value), byteorder = "little", signed = False)
        return length+super().to_bytes(value)


class CStr(BaseStr):
    @classmethod
    def from_generator(cls, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little") -> str:
        length = int.from_bytes(igen.get_bytes(4), "little", signed = False)
        return cls.from_bytes(igen.get_bytes(length)[:-1])

    @classmethod
    def to_bytes(cls, value: str, byteorder: Literal["big", "little"] = "little") -> bytes:
        length = int.to_bytes(len(value)+1, length = 4, byteorder = "little", signed = False)
        return length+super().to_bytes(value)+b"\x00"


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
        if cmp(len(sizable)) >= length:
            break
    else:
        return True, ""
    return False, f"each element of %s must have a fixed length {cmp.__name__} {length}"
