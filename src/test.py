from bfp_rs.types import Version, BfpType
from bfp_rs.types.le import int8, int16, float64
from bfp_rs import Retriever, BaseStruct, Struct

class Test(BaseStruct):
    a = Retriever(BfpType.Int8(int8()))
    b = Retriever(BfpType.Int8(int8()))

class Test2(BaseStruct):
    b = Retriever(Struct[Test])

print(Struct[Test])
