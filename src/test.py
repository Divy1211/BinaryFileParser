from bfp_rs.types import Version
from bfp_rs.types.le import u8, bool8, Bytes, void, str8, Encoding, Str, NtStr, c_str, nt_str8, str_array8
from bfp_rs.combinators import set, if_, if_not, if_len, set_repeat
from bfp_rs import Retriever, BaseStruct, ByteStream

from utils import timed

class SubTest(BaseStruct):
    str2 = Retriever(u8)

class Test(BaseStruct):
    sub_test = Retriever(SubTest)
    str2 = Retriever(str_array8)

# test = Test.from_bytes(b"hello world\x00")
test = Test.from_bytes(b"\x05\x02\x04\x03poggers")
print(test.sub_test.str2)
print(test.str2)


