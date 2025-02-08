from bfp_rs.types import BfpType, Version
from bfp_rs.types.le import int8, bool8
from bfp_rs.combinators import set, if_, if_not, if_len
from bfp_rs import Retriever, BaseStruct, ByteStream, Struct

from utils import timed

class TestSub(BaseStruct):
    num1 = Retriever(BfpType.Int8(int8()))
    num2 = Retriever(BfpType.Int8(int8()), repeat = 5)

class Test(BaseStruct):
    num1 = Retriever(BfpType.Int8(int8()))
    num2 = Retriever(BfpType.Int8(int8()))
    test_sub = Retriever(Struct[TestSub])
    num3 = Retriever(BfpType.Int8(int8()), on_read = lambda: [if_len(Test.test_sub, TestSub.num2).leq(Test.test_sub, TestSub.num2, 0).then(set(Test.test_sub, TestSub.num1).from_len(Test.test_sub, TestSub.num2))])

test = Test.from_bytes(bytes(range(32)))
print(test.test_sub.num1)
