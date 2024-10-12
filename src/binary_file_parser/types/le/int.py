import struct

from binary_file_parser.types.parseable import Parseable
from binary_file_parser.types.byte_stream import ByteStream
from binary_file_parser.types.version import Version


class Int(Parseable):
    __slots__ = "struct_symbol"

    def __init__(self, size: int, struct_symbol: str):
        super().__init__(size)
        self.struct_symbol = struct_symbol

    def _from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> int:
        return self._from_bytes(stream.get(self._size), struct_ver = struct_ver)

    def _from_bytes(self, bytes_: bytes, *, struct_ver: Version = Version((0,))) -> int:
        return struct.unpack(self.struct_symbol, bytes_)[0]

    def _to_bytes(self, value: int) -> bytes:
        return struct.pack(self.struct_symbol, value)

int8 = Int(1, "<b")
int16 = Int(2, "<h")
int32 = Int(4, "<i")
int64 = Int(8, "<q")
uint8 = Int(1, "<B")
uint16 = Int(2, "<H")
uint32 = Int(4, "<I")
uint64 = Int(8, "<Q")
