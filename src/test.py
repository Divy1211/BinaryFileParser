from bfp_rs.types import Version
from bfp_rs.types.le import u8, bool8, Bytes, void, str8, Encoding, Str, NtStr, c_str, nt_str8, str_array8, Option8, Array8, Array, StackedArray, StackedArray8, StackedAttrArray8, StackedAttrArray
from bfp_rs.combinators import set, if_, if_not, if_len, set_repeat, get
from bfp_rs import Retriever, BaseStruct, ByteStream

from utils import timed

class SubTest(BaseStruct):
    num1 = Retriever(u8, default = 1)
    num2 = Retriever(u8, default = 2)

    def __str__(self):
        return f"SubTest({self.num1}, {self.num2})"

class Test(BaseStruct):
    nums1 = Retriever(u8)
    nums = Retriever(SubTest, on_read = lambda: [set_repeat(Test.nums3).by(0b0011 & get(Test.nums, SubTest.num1) | get(Test.nums, SubTest.num2) * 2)])
    nums3 = Retriever(u8)


test = Test.from_bytes(b"\x01\x03\x04\x03\x04\x03\x04\x04\x04\x04\x04\x04\x04\x03\x04\x03\x04\x04\x04\x04\x04\x04\x04")
