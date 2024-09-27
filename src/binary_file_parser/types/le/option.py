from __future__ import annotations

import struct
from typing import Generic, Type, TYPE_CHECKING, TypeVar

from binary_file_parser.types.parseable import Parseable
from binary_file_parser.types.version import Version
from binary_file_parser.types.byte_stream import ByteStream

if TYPE_CHECKING:
    ParseableType = Type[Parseable] | Parseable

T = TypeVar("T")
class Option(Parseable, Generic[T]):
    __slots__ = ("dtype", "struct_symbol")

    def __init__(self, size: int, dtype: ParseableType, struct_symbol: str):
        super().__init__(size)
        self.dtype = dtype
        self.struct_symbol = struct_symbol

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> T | None:
        exists = struct.unpack(self.struct_symbol, stream.get(self._size))[0] != 0
        if not exists:
            return None
        return self.dtype._from_stream(stream, struct_ver = struct_ver)

    def _from_bytes(self, bytes_: bytes, *, struct_ver: Version = Version((0,))):
        return self._from_stream(ByteStream.from_bytes(bytes_), struct_ver = struct_ver)

    def _to_bytes(self, value: T | None) -> bytes:
        if value is None:
            return struct.pack(self.struct_symbol, 0)
        return struct.pack(self.struct_symbol, 1) + self.dtype._to_bytes(value)

class Option8(Option):
    """
    Represents an optional type, the inner type is read only if the following ``uint8`` is non-zero
    Usage:

    >>> Option8[int32]
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType) -> Option8:
        return cls(1, item, '<B')

class Option16(Option):
    """
    Represents an optional type, the inner type is read only if the following ``uint16`` is non-zero
    Usage:

    >>> Option16[int32]
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType) -> Option16:
        return cls(2, item, '<H')

class Option32(Option):
    """
    Represents an optional type, the inner type is read only if the following ``uint32`` is non-zero
    Usage:

    >>> Option32[int32]
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType) -> Option32:
        return cls(4, item, '<I')

class Option64(Option):
    """
    Represents an optional type, the inner type is read only if the following ``uint64`` is non-zero
    Usage:

    >>> Option64[int32]
    """
    __slots__ = ()

    def __class_getitem__(cls, item: ParseableType) -> Option64:
        return cls(8, item, '<Q')