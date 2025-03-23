from bfp_rs.types.le import u8, bool8, Bytes, void, str8, Encoding, Str, NtStr, c_str, nt_str8, str_array8, Option8, Array8, Array, StackedArray, StackedArray8, StackedAttrArray8, StackedAttrArray
from bfp_rs.combinators import set_, if_, if_not, if_len, set_repeat, get
from bfp_rs import Retriever, BaseStruct, ByteStream, Version, RetrieverRef, RetrieverCombiner, Manager

from utils import timed

class SubTest(BaseStruct):
    a = Retriever(u8, repeat = 2)

    def __str__(self):
        return f"SubTest({self.a[0]}, {self.a[1]})"

class Test(BaseStruct):
    a = Retriever(u8, max_ver = Version(1), repeat = 2)

    sub = Retriever(SubTest, min_ver = Version(2))

    a0 = RetrieverRef(a, 0)
    a1 = RetrieverRef(a, 1)

    sub_a0 = RetrieverRef(sub, SubTest.a, 0)
    sub_a1 = RetrieverRef(sub, SubTest.a, 1)

    com_a0 = RetrieverCombiner(a0, sub_a0)
    com_a1 = RetrieverCombiner(a1, sub_a1)

    @classmethod
    def _get_version(
        cls,
        stream: ByteStream,
        _ver: Version = Version(0),
    ) -> Version:
        return Version(1)

    def __new__(cls, ver = Version(-1), init_defaults = True, **retriever_inits):
        self = super().__new__(cls, ver, init_defaults, **retriever_inits)
        self.test_man = TestMan(self)
        return self

class TestMan(Manager):
    a = RetrieverRef(Test.com_a0)

test = Test.from_bytes(b"\x01\x02")

print(test.test_man.a)

