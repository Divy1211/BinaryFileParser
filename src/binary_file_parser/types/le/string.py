from __future__ import annotations

import struct
from abc import ABC
from functools import partial
from itertools import takewhile
from operator import ne
from typing import Type, TYPE_CHECKING

from binary_file_parser.types.byte_stream import ByteStream
from binary_file_parser.types.parseable import Parseable
from binary_file_parser.types.version import Version


if TYPE_CHECKING:
    ParseableType = Type[Parseable] | Parseable

class BaseStr(Parseable, ABC):
    __slots__ = ()

    def _from_bytes(self, bytes_: bytes, *, struct_ver: Version = Version((0,))) -> str:
        try:
            return bytes_.decode("utf-8")
        except UnicodeDecodeError:
            return bytes_.decode("latin-1")

    def _to_bytes(self, value: str) -> bytes:
        try:
            return value.encode("utf-8")
        except UnicodeEncodeError:
            return value.encode("latin-1")

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


class StrArray(BaseStr):
    __slots__ = "struct_symbol", "num_strings"

    def __init__(self, size: int, struct_symbol: str, num_strings: int = -1):
        super().__init__(size)
        self.struct_symbol = struct_symbol
        self.num_strings = num_strings

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> list[str]:
        num_strings = self.num_strings
        if num_strings == -1:
            num_strings = struct.unpack(self.struct_symbol, stream.get(self._size))[0]

        lengths: list[int] = [struct.unpack(self.struct_symbol, stream.get(self._size))[0] for _ in range(num_strings)]
        ls: list[str] = [""]*num_strings

        for i, length in enumerate(lengths):
            ls[i] = super()._from_bytes(stream.get(length), struct_ver = struct_ver)

        return ls

    def _to_bytes(self, value: list[str]) -> bytes:
        if self.num_strings != -1 and len(value) != self.num_strings:
            raise TypeError(f"Expected {self.num_strings} StackedStrings, found {len(value)}")

        length_bytes = b""
        num_strings = self.num_strings
        if num_strings == -1:
            num_strings = len(value)
            length_bytes = struct.pack(self.struct_symbol, num_strings)

        bytes_: list[bytes] = [b""]*(2*num_strings)
        for i in range(num_strings):
            bytes_[num_strings+i] = super()._to_bytes(value[i])
            bytes_[i] = struct.pack(self.struct_symbol, len(bytes_[num_strings+i]))

        return length_bytes+b"".join(bytes_)

    def __getitem__(self, item: int) -> StrArray:
        return self.__class__(self._size, self.struct_symbol, item)


c_str = CStr(4)
"""A C style null terminated string"""
str8 = Str(1, "<B")
"""A string whose length in bytes is indicated by a ``uint8`` at the start"""
str16 = Str(2, "<H")
"""A string whose length in bytes is indicated by a ``uint16`` at the start"""
str32 = Str(4, "<I")
"""A string whose length in bytes is indicated by a ``uint32`` at the start"""
str64 = Str(8, "<Q")
"""A string whose length in bytes is indicated by a ``uint64`` at the start"""
nt_str8 = NullTermStr(1, "<B")
"""Same as ``str8`` but removes a null terminated character from the end of the string if present.
Also appends one on write if one is not already present"""
nt_str16 = NullTermStr(2, "<H")
"""Same as ``str16`` but removes a null terminated character from the end of the string if present.
Also appends one on write if one is not already present"""
nt_str32 = NullTermStr(4, "<I")
"""Same as ``str32`` but removes a null terminated character from the end of the string if present.
Also appends one on write if one is not already present"""
nt_str64 = NullTermStr(8, "<Q")
"""Same as ``str64`` but removes a null terminated character from the end of the string if present.
Also appends one on write if one is not already present"""
StrArray8 = StrArray(1, '<B', -1)
"""Represents a list of strings"""
StrArray16 = StrArray(2, '<H', -1)
StrArray32 = StrArray(4, '<I', -1)
StrArray64 = StrArray(8, '<Q', -1)
