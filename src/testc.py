from __future__ import annotations

from bfp_rs import BaseStruct, Retriever, RetrieverRef
from bfp_rs.types.le import i16, bool8, Array


class BaseClass(BaseStruct):
    # @formatter:off
    mask: int         = Retriever(i16,                          default = 0)
    type: int         = Retriever(i16,                          default = 0)
    density: int      = Retriever(i16,                          default = 0)
    centralized: bool = Retriever(bool8,                        default = False)
    # @formatter:on

class SubClass(BaseClass):
    uwu: int = Retriever(i16, default = 0)

    test = RetrieverRef(BaseClass.mask)

# a = SubClass(type = 20, mask = 20)
# a.to_json(r"../test.json")
a = SubClass.from_json(r"../test.json")
print(a.mask)
