from bfp_rs.types import BfpType, Version
from bfp_rs.types.le import int8, bool8
from bfp_rs.combinators import set_repeat, if_, if_not, if_len, set
from bfp_rs import Retriever, BaseStruct, ByteStream, Struct

from utils import timed


class Test(BaseStruct):
    num1 = Retriever(BfpType.Bool8(bool8()))
    num2 = Retriever(BfpType.Int8(int8()))
    num3 = Retriever(BfpType.Int8(int8()), repeat = 10, on_read = lambda: [set(Test.num2).from_len(Test.num3)])

test = Test.from_bytes(bytes(range(32)))
print(test.num2, test.num3)
