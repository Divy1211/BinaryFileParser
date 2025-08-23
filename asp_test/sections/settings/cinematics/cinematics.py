from __future__ import annotations

from bfp_rs import BaseStruct, Retriever, Version, Context
from bfp_rs.types.le import str16

from asp_test.sections.scx_versions import DE_LATEST


class Cinematics(BaseStruct):
    # @formatter:off
    pregame: str = Retriever(str16, default = "")
    victory: str = Retriever(str16, default = "")
    loss: str    = Retriever(str16, default = "")
    # @formatter:on

    def __new__(cls, ver: Version = DE_LATEST, ctx: Context = None, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, ctx or Context(), init_defaults, **retriever_inits)
