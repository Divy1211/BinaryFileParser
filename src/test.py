from bfp_rs.types.le import (
    u8, bool8, Bytes, void, str8, Encoding, Str, NtStr, c_str, nt_str8, str_array8, Option8,
    Array8, Array, StackedArray, StackedArray8, StackedAttrArray8, StackedAttrArray, Tail
)
from bfp_rs.combinators import set_, if_, if_not, if_len, set_repeat, get, set_key, if_key, if_not_key, get_key
from bfp_rs import Retriever, BaseStruct, ByteStream, Version, RetrieverRef, RetrieverCombiner

from utils import timed

class SubTest(BaseStruct):
    void = Retriever(void, on_read = lambda: [if_key("num_nums").ge(2).then(set_repeat(SubTest.nums).from_key("num_nums"))])
    nums = Retriever(u8, repeat = 0)

class Test(BaseStruct):
    ls = Retriever(Array8[u8], on_read = lambda: [set_key("num_nums").from_len(Test.ls)])
    sub = Retriever(SubTest)

test = Test.from_bytes(b"\x04\x01\x02\x03\x04\x01\x02\x03\x05")

print(test.ls)
print(test.sub.nums)
