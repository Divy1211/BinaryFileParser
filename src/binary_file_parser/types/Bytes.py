from __future__ import annotations

from binary_file_parser.types.ByteStream import ByteStream
from binary_file_parser.types.Parseable import Parseable
from binary_file_parser.utils import Version


class Bytes(Parseable):
    __slots__ = ()

    def is_valid(self, value: bytes) -> tuple[bool, str]:
        if len(value) == self.size:
            return True, ""
        return False, f"number of bytes in %s must equal {self.size}"

    def from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> bytes:
        return stream.get(self.size)

    def from_bytes(self, bytes_: bytes, *, struct_ver: Version = Version((0,))) -> bytes:
        return bytes_

    def to_bytes(self, value: bytes) -> bytes:
        return value

    def __class_getitem__(cls, item: int) -> Bytes:
        return cls(item)
