from __future__ import annotations

from bfp_rs import BaseStruct, Retriever, Version, Context
from bfp_rs.types.le import i32

from asp_test.sections.scx_versions import DE_LATEST


class View(BaseStruct):
    x: int = Retriever(i32, default = -1)
    y: int = Retriever(i32, default = -1)

    def __new__(cls, ver: Version = DE_LATEST, ctx: Context = None, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, ctx or Context(), init_defaults, **retriever_inits)
