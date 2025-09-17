from bfp_rs import ret, BaseStruct, RefStruct, Retriever, RetrieverRef, Context, Version
from bfp_rs.combinators import get_attr, get
from bfp_rs.types.le import u8


class Test(BaseStruct):
    offset: int     = Retriever(u8, default = 1)
    nums: list[int] = Retriever(u8, default = 1, repeat = 10)

class RefTest(RefStruct):
    _struct: Test
    val = RetrieverRef(ret(Test.nums), get_attr("index") % 8)

    def __new__(cls, struct: Test, index: int):
        self = super().__new__(cls, struct)
        self.index = index
        return self

test = Test.from_bytes(b"\x02"+bytes(range(10)))
ref = RefTest(test, -1)
print(test.offset, test.nums)
print(ref.val)
