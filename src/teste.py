from enum import Enum, IntEnum

from bfp_rs import ret, BaseStruct, RefStruct, Retriever, RetrieverRef, Context, Version
from bfp_rs.combinators import get_attr, get, set_key, set_repeat
from bfp_rs.types.le import u8, Str


class Test(BaseStruct):
    offset: int     = Retriever(u8, default = 1, on_read = lambda: [set_repeat(ret(Test.nums)).from_key("test")])
    nums: list[int] = Retriever(u8, default = 1, repeat = 0)

test = Test(ctx = Context(test = (u8, 3))) #.from_bytes(b"\x02"+bytes(range(2)))

print(test.nums)
