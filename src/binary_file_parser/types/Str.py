from __future__ import annotations

import struct
from abc import ABC

from binary_file_parser.types.ByteStream import ByteStream
from binary_file_parser.types.Parseable import Parseable
from binary_file_parser.utils import Version


class BaseStr(Parseable, ABC):
    __slots__ = ()

    def from_bytes(self, bytes_: bytes, *, struct_version: Version = Version((0,))) -> str:
        try:
            return bytes_.decode("utf-8")
        except UnicodeDecodeError:
            return bytes_.decode("latin-1")

    def to_bytes(self, value: str) -> bytes:
        # TODO: plain bytes
        try:
            bytes_ = value.encode("utf-8")
        except UnicodeEncodeError:
            bytes_ = value.encode("latin-1")

        return bytes_


class CStr(BaseStr):
    __slots__ = ()

    def from_stream(self, stream: ByteStream, *, struct_version: Version = Version((0,))) -> str:
        bytes_ = b""
        while (byte := stream.get(1)) != b"\x00":
            bytes_ += byte
        return self.from_bytes(bytes_)

    def to_bytes(self, value: str) -> bytes:
        if not value.endswith("\x00"):
            value += "\x00"
        return super().to_bytes(value)


class Str(BaseStr):
    __slots__ = "struct_symbol"

    def __init__(self, size: int, struct_symbol: str):
        super().__init__(size)
        self.struct_symbol = struct_symbol

    def from_stream(self, stream: ByteStream, *, struct_version: Version = Version((0,))) -> str:
        length: int = struct.unpack(self.struct_symbol, stream.get(self.size))[0]
        return self.from_bytes(stream.get(length))

    def to_bytes(self, value: str) -> bytes:
        bytes_ = super().to_bytes(value)
        length = struct.pack(self.struct_symbol, len(bytes_))
        return length+bytes_


class NullTermStr(Str):
    __slots__ = ()

    def from_stream(self, stream: ByteStream, *, struct_version: Version = Version((0,))) -> str:
        return super().from_stream(stream, struct_version = struct_version)[:-1]

    def to_bytes(self, value: str) -> bytes:
        if not value.endswith("\x00"):
            value += "\x00"
        return super().to_bytes(value)


class FixedLenStr(BaseStr):
    __slots__ = "length"

    def __init__(self, size: int, length: int):
        super().__init__(size)
        self.length = length

    def is_valid(self, value: str) -> tuple[bool, str]:
        if len(value) == self.length:
            return True, ""
        return False, f"%s must have a fixed length of {value}"

    def from_stream(self, stream: ByteStream, *, struct_version: Version = Version((0,))) -> str:
        return self.from_bytes(stream.get(self.length))

    def __class_getitem__(cls, item: int) -> FixedLenStr:
        return cls(4, item)

c_str = CStr(4)
str8 = Str(1, "<B")
str16 = Str(2, "<H")
str32 = Str(4, "<I")
str64 = Str(8, "<Q")
nt_str8 = NullTermStr(1, "<B")
nt_str16 = NullTermStr(2, "<H")
nt_str32 = NullTermStr(4, "<I")
nt_str64 = NullTermStr(8, "<Q")
