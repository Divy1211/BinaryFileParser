from bfp_rs.types import BfpType
from bfp_rs.types.le import int8
from bfp_rs import Retriever, BaseStruct, ByteStream

class Test(BaseStruct):
    ls = Retriever(BfpType.Int8(int8()), repeat = 50)

test = Test.test_from_stream(ByteStream.from_bytes(bytes(range(50))))

test.ls = []
print(test.ls)
