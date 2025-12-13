from enum import Enum, IntEnum

from bfp_rs import ret, BaseStruct, RefStruct, Retriever, RetrieverRef, Context, Version
from bfp_rs.combinators import get_attr, get, set_key, set_repeat
from bfp_rs.diff import NestedDiff
from bfp_rs.types.le import u8, Str, Option8


class Test(BaseStruct):
    offset: int | None = Retriever(Option8[u8], default = 1)

a = Test.from_bytes(b"\x01\x04")
b = Test.from_bytes(b"\x01\x05")

print(a.diff(b))
