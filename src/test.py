from bfp_rs.types import Version
from bfp_rs.types.le import u8, bool8, Bytes, void, str8, Encoding
from bfp_rs.combinators import set, if_, if_not, if_len, set_repeat
from bfp_rs import Retriever, BaseStruct, ByteStream, Struct

from utils import timed

class Test(BaseStruct):
    num1 = Retriever(str8)

test = Test.from_bytes(b"\x0bhello world")
print(test.num1)

print(str8.to_bytes("1"))
