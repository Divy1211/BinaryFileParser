from bfp_rs.types import Version, BfpType
from bfp_rs.types.le import int8, int16, float64
from bfp_rs import Retriever, BaseStruct, Struct, ByteStream

class Test(BaseStruct):
    one = Retriever(BfpType.Int8(int8()))
    two = Retriever(BfpType.Int8(int8()))

class Test2(BaseStruct):
    test = Retriever(Struct[Test])

test = Test2.from_stream(ByteStream.from_bytes(b"\x00\x00"))
