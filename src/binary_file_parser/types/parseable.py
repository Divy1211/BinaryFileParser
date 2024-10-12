from abc import ABC, abstractmethod
from typing import TypeVar

from binary_file_parser.types.byte_stream import ByteStream
from binary_file_parser.types.version import Version

T = TypeVar("T")
class Parseable(ABC):
    __slots__ = "_size"

    def __init__(self, size: int):
        self._size = size

    @abstractmethod
    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> T:
        ...

    @abstractmethod
    def _from_bytes(self, bytes_: bytes, *, struct_ver: Version = Version((0,))) -> T:
        ...

    @abstractmethod
    def _to_bytes(self, value: T) -> bytes:
        ...
