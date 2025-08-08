from __future__ import annotations

from bfp_rs import BaseStruct, Retriever
from bfp_rs.types.le import i16, bool8, StackedAttrArray


class TerrainUnit(BaseStruct):
    # @formatter:off
    mask: int         = Retriever(i16,                          default = 0)
    type: int         = Retriever(i16,                          default = 0)
    density: int      = Retriever(i16,                          default = 0)
    centralized: bool = Retriever(bool8,                        default = False)
    # @formatter:on

class Test(BaseStruct):
    arr = Retriever(StackedAttrArray[4][TerrainUnit], default_factory = lambda _: [TerrainUnit() for _ in range(4)])

# by = bytes([*[1]*8, *[2]*8, *[3]*8, *[1]*4])

# print(by)

# a = Test.from_bytes(by)

# by = Test.to_bytes(a)

# print(by)

a = Test()

print(a.arr)
