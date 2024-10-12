from bfp_rs.types import BfpType, Version
from bfp_rs.types.le import int8
from bfp_rs.combinators import set_repeat, if_, if_not
from bfp_rs import Retriever, BaseStruct, ByteStream, Struct

from utils import timed


class Test(BaseStruct):
    num_nums = Retriever(BfpType.Int8(int8()), on_read = lambda: [if_not(Test.num_nums).eq(17).then(set_repeat(Test.nums).to(5))])
    nums = Retriever(BfpType.Int8(int8()))

test = Test.from_bytes(b"\x10\x01\x01\x01\x01\x01")
print(test.nums)
