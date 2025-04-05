from bfp_rs.types.le import (
    u8, bool8, Bytes, void, str8, Encoding, Str, NtStr, c_str, nt_str8, str_array8, Option8,
    Array8, Array, StackedArray, StackedArray8, StackedAttrArray8, StackedAttrArray, Tail
)
from bfp_rs.combinators import set_, if_, if_not, if_len, set_repeat, get
from bfp_rs import Retriever, BaseStruct, ByteStream, Version, RetrieverRef, RetrieverCombiner

from utils import timed

class SubTest(BaseStruct):
    a = Retriever(Tail[u8])


test = SubTest.from_bytes(b"\x01\x01\x02\x03\x04")

print(test.a)
