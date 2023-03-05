from abc import ABC, abstractmethod
from typing import TypeVar

from binary_file_parser.types.ByteStream import ByteStream

T = TypeVar("T")
class Parseable(ABC):
    __slots__ = "size"

    def __init__(self, size: int):
        self.size = size

    @staticmethod
    def is_valid(value: T) -> tuple[bool, str]:
        return True, ""

    @property
    def is_struct(self) -> bool:
        return False

    @abstractmethod
    def from_stream(self, stream: ByteStream, *, struct_version: tuple[int, ...] = (0,)) -> T:
        ...

    @abstractmethod
    def from_bytes(self, bytes_: bytes, *, struct_version: tuple[int, ...] = (0,)) -> T:
        ...

    @abstractmethod
    def to_bytes(self, value: T) -> bytes:
        ...
