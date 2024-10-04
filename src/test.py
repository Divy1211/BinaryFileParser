from bfp_rs.types import Version, BfpType
from bfp_rs.types.le import int8, int16, float64
from bfp_rs import Retriever, BaseStruct

class Test(BaseStruct):
    a = Retriever(BfpType.Int8(int8()))

class Test2:
    a = Retriever(BfpType.Int8(int8()))

print(Test.retrievers, Test2.retrievers)