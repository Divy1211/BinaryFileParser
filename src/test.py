from bfp_rs.types import BfpType, Version
from bfp_rs.types.le import int8, bool8
from bfp_rs.combinators import set, if_, if_not, if_len, set_repeat
from bfp_rs import Retriever, BaseStruct, ByteStream, Struct

from utils import timed

class TestSub(BaseStruct):
    num1 = Retriever(BfpType.Int8(int8()))
    num2 = Retriever(BfpType.Int8(int8()), repeat = 5)

class Test(BaseStruct):
    num1 = Retriever(BfpType.Int8(int8()))
    num2 = Retriever(BfpType.Int8(int8()))
    test_sub = Retriever(Struct[TestSub], on_read = lambda: [set_repeat(Test.num3).to(10)])
    num3 = Retriever(BfpType.Int8(int8()))

test = Test.from_bytes(bytes(range(32)))
print(test.num3)
