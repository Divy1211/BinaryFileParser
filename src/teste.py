from enum import Enum, IntEnum

from bfp_rs import ret, BaseStruct, RefStruct, Retriever, RetrieverRef, Context, Version, BfpList
from bfp_rs.combinators import get_attr, get, set_key, set_repeat, set_, if_
from bfp_rs.diff import NestedDiff
from bfp_rs.types.le import u8, Str, Option8, u32, i8


def set_nums_repeat():
    return [
        if_(ret(Test.num_nums)).eq(-1).then(set_(ret(Test.num_nums)).to(0)),
        set_repeat(ret(Test.nums)).from_(ret(Test.num_nums))
    ]

def sync_len():
    return [
        set_(ret(Test.num_nums)).from_len(ret(Test.nums))
    ]

class Test(BaseStruct):
    num_nums: int = Retriever(i8, default = -1, on_read = set_nums_repeat, on_write = sync_len)
    nums: int | None = Retriever(u8, default_factory = lambda _ver: [], repeat = 0)

a = Test()

print(isinstance(a.nums, BfpList))
