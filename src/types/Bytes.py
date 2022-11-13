from typing import Literal

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.types.ParserType import ParserType


class Bytes(ParserType):
    __slots__ = "num_bytes",

    def __init__(self, num_bytes: int):
        self.num_bytes = num_bytes

    def is_valid(self, value: bytes) -> tuple[bool, str]:
        if len(value) == self.num_bytes:
            return True, ""
        return False, f"number of bytes in %s must equal {self.num_bytes}"

    def from_generator(self, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> bytes:
        return igen.get_bytes(self.num_bytes)

    def from_bytes(self, bytes_: bytes, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> bytes:
        return bytes_[:self.num_bytes]

    def to_bytes(self, value: bytes, byteorder: Literal["big", "little"] = "little") -> bytes:
        return value
