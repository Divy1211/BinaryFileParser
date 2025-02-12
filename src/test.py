from bfp_rs.types import Version
from bfp_rs.types.le import u8, bool8, Bytes, void, str8, Encoding, Str, NtStr, c_str, nt_str8, str_array8, Option8, Array8, Array
from bfp_rs.combinators import set, if_, if_not, if_len, set_repeat
from bfp_rs import Retriever, BaseStruct, ByteStream

from utils import timed

class SubTest(BaseStruct):
    num = Retriever(u8)

class Test(BaseStruct):
    nums = Retriever(Array[3][u8])

test = Test.from_bytes(b"\x02\x01\x04")

print(test.nums)
