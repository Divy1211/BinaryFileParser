from bfp_rs.types import Version
from bfp_rs.types.le import u8, bool8, Bytes, void, str8, Encoding, Str, NtStr, c_str, nt_str8, str_array8, Option8, Array8, Array, StackedArray, StackedArray8
from bfp_rs.combinators import set, if_, if_not, if_len, set_repeat
from bfp_rs import Retriever, BaseStruct, ByteStream

from utils import timed

class SubTest(BaseStruct):
    num = Retriever(u8)

class Test(BaseStruct):
    nums = Retriever(StackedArray8[2][u8])

test = Test.from_bytes(b"\x01\x04\x03\x04\x03\x06\x07")

# print(test.nums)

test.nums = [[1], [2, 3]]

print(StackedArray8[2][u8].to_bytes([[1], [2, 3], [2]]))
