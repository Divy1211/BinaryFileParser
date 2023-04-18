import struct

from binary_file_parser.types.ByteStream import ByteStream
from binary_file_parser.types.Parseable import Parseable
from binary_file_parser.utils import Version


class Float(Parseable):
    __slots__ = "struct_symbol"

    def __init__(self, size: int, struct_symbol: str):
        super().__init__(size)
        self.struct_symbol = struct_symbol

    def from_stream(self, stream: ByteStream, *, struct_version: Version = Version((0,))) -> float:
        return self.from_bytes(stream.get(self.size))

    def from_bytes(self, bytes_: bytes, *, struct_version: Version = Version((0,))) -> float:
        return struct.unpack(self.struct_symbol, bytes_)[0]

    def to_bytes(self, value: float) -> bytes:
        return struct.pack(self.struct_symbol, value)

float16 = Float(2, "e")
float32 = Float(4, "f")
float64 = Float(8, "d")
