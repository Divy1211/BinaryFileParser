from bfp_rs.types.le import i8
from bfp_rs.combinators import set_repeat, if_, if_not, if_len, if_ver_min, if_ver_max, if_ver_in
from bfp_rs import Retriever, BaseStruct, ByteStream, Version

from utils import timed

class Test(BaseStruct):
    one = Retriever(i8, on_read = lambda: [if_ver_in(Version(1), Version(2)).then(set_repeat(Test.two).to(2))])
    two = Retriever(i8)

    @classmethod
    def _get_version(
        cls,
        stream: ByteStream,
        _ver: Version = Version(0),
    ) -> Version:
        return Version(i8.from_bytes(stream.peek(1)))

test = Test.from_bytes(b"\x03\x01\x02")
print(test.two)
