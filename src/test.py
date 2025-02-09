from bfp_rs.types import Version
from bfp_rs.types.le import u8, bool8, Bytes, void
from bfp_rs.combinators import set, if_, if_not, if_len, set_repeat
from bfp_rs import Retriever, BaseStruct, ByteStream, Struct

from utils import timed

class TestSub(BaseStruct):
    num1 = Retriever(u8)
    num2 = Retriever(u8, repeat = 5)

class Test(BaseStruct):
    num1 = Retriever(void)
    num2 = Retriever(u8)
    test_sub = Retriever(Struct[TestSub], on_read = lambda: [set_repeat(Test.num3).to(10)])
    num3 = Retriever(u8)

test = Test.from_bytes(bytes(range(32)))
print(test.num2)
