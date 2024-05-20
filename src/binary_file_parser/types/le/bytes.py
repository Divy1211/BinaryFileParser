from __future__ import annotations

from binary_file_parser.types.byte_stream import ByteStream
from binary_file_parser.types.parseable import Parseable
from binary_file_parser.types.version import Version


class Bytes(Parseable):
    __slots__ = ()

    def is_valid(self, value: bytes) -> tuple[bool, str]:
        if len(value) == self._size:
            return True, ""
        return False, f"number of bytes in %s must equal {self._size}"

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> bytes:
        return stream.get(self._size)

    def _from_bytes(self, bytes_: bytes, *, struct_ver: Version = Version((0,))) -> bytes:
        return bytes_

    def _to_bytes(self, value: bytes) -> bytes:
        return value

    def __class_getitem__(cls, size: int) -> Bytes:
        return cls(size)
