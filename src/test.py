from bfp_rs.types import BfpType, Version
from bfp_rs.types.le import int8, bool8
from bfp_rs.combinators import set_repeat, if_, if_not, if_len
from bfp_rs import Retriever, BaseStruct, ByteStream, Struct

from utils import timed


class Test(BaseStruct):
    num = Retriever(BfpType.Bool8(bool8()))
    nums1 = Retriever(BfpType.Int8(int8()), repeat = 16, on_read = lambda: [if_len(Test.nums1).gt(1).then(set_repeat(Test.nums2).to(15))])
    nums2 = Retriever(BfpType.Int8(int8()))

test = Test.from_bytes(bytes(range(1, 33)))
print(test.nums1, test.nums2)
