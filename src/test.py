from bfp_rs.types.le import (
    u8, bool8, Bytes, void, str8, Encoding, Str, NtStr, c_str, nt_str8, str_array8, Option8,
    Array8, Array, StackedArray, StackedArray8, StackedAttrArray8, StackedAttrArray, Tail
)
from bfp_rs.combinators import set_, if_, if_not, if_len, set_repeat, get, set_key, if_key, if_not_key, get_key, if_else, break_
from bfp_rs import Retriever, BaseStruct, ByteStream, Version, RetrieverRef, RetrieverCombiner, Context

from utils import timed

def test():
    return [
        if_else(
            if_(Test.len).eq(1).then(set_repeat(Test.nums).to(1)),
            if_(Test.len).ge(2).then(if_(Test.len).le(3).then(set_repeat(Test.nums).to(3))),
            if_(Test.len).ge(4).then(if_(Test.len).ge(5).then(set_repeat(Test.nums).to(5)), break_()),
            set_repeat(Test.nums).to(0)
        )
    ]

class Test(BaseStruct):
    len = Retriever(u8, on_read = test)
    nums = Retriever(u8, repeat = -2)

test = Test.from_bytes(b"\x05\x01\x02\x03\x04\x05\x06")

print(test.nums)