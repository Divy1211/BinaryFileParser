from __future__ import annotations

import struct
from abc import ABC
from functools import partial
from itertools import takewhile
from operator import ne

from binary_file_parser.types.byte_stream import ByteStream
from binary_file_parser.types.parseable import Parseable
from binary_file_parser.types.version import Version


class BaseStr(Parseable, ABC):
    __slots__ = ()

    def _from_bytes(self, bytes_: bytes, *, struct_ver: Version = Version((0,))) -> str:
        try:
            return bytes_.decode("utf-8")
        except UnicodeDecodeError:
            return bytes_.decode("latin-1")

    def _to_bytes(self, value: str) -> bytes:
        # TODO: plain bytes
        try:
            bytes_ = value.encode("utf-8")
        except UnicodeEncodeError:
            bytes_ = value.encode("latin-1")

        return bytes_


class CStr(BaseStr):
    __slots__ = ()

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> str:
        bytes_ = b""
        while (byte := stream.get(1)) != b"\x00":
            bytes_ += byte
        return self._from_bytes(bytes_)

    def _to_bytes(self, value: str) -> bytes:
        if not value.endswith("\x00"):
            value += "\x00"
        return super()._to_bytes(value)


class Str(BaseStr):
    __slots__ = "struct_symbol"

    def __init__(self, size: int, struct_symbol: str):
        super().__init__(size)
        self.struct_symbol = struct_symbol

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> str:
        length: int = struct.unpack(self.struct_symbol, stream.get(self._size))[0]
        return self._from_bytes(stream.get(length))

    def _to_bytes(self, value: str) -> bytes:
        bytes_ = super()._to_bytes(value)
        length = struct.pack(self.struct_symbol, len(bytes_))
        return length+bytes_


class NullTermStr(Str):
    __slots__ = ()

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> str:
        return super()._from_stream(stream, struct_ver = struct_ver).removesuffix("\x00")

    def _to_bytes(self, value: str) -> bytes:
        if not value.endswith("\x00"):
            value += "\x00"
        return super()._to_bytes(value)


class FixedLenStr(BaseStr):
    __slots__ = "length"

    def __init__(self, size: int, length: int):
        super().__init__(size)
        self.length = length

    def _to_bytes(self, value: str) -> bytes:
        if len(value) != self.length:
            raise TypeError(f"Expected FixedLenStr[{self.length}], found string with length: {len(value)}")
        return super()._to_bytes(value)

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> str:
        return self._from_bytes(stream.get(self.length))

    def __class_getitem__(cls, item: int) -> FixedLenStr:
        return cls(4, item)

class FixedLenNTStr(FixedLenStr):
    __slots__ = ()

    def _to_bytes(self, value: str) -> bytes:
        value = f"{value:\x00<{self.length}}"
        return super()._to_bytes(value)

    def _from_bytes(self, bytes_: bytes, *, struct_ver: Version = Version((0,))) -> str:
        value = super()._from_bytes(bytes_, struct_ver = struct_ver)
        return str(takewhile(partial(ne, "\x00"), value))


c_str = CStr(4)
str8 = Str(1, "<B")
str16 = Str(2, "<H")
str32 = Str(4, "<I")
str64 = Str(8, "<Q")
nt_str8 = NullTermStr(1, "<B")
nt_str16 = NullTermStr(2, "<H")
nt_str32 = NullTermStr(4, "<I")
nt_str64 = NullTermStr(8, "<Q")
