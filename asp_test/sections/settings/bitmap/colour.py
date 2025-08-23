from bfp_rs import BaseStruct, Retriever, Version, Context
from bfp_rs.types.le import u8

from asp_test.sections.scx_versions import DE_LATEST


class Colour(BaseStruct):
    blue: int     = Retriever(u8, default = 0)
    green: int    = Retriever(u8, default = 0)
    red: int      = Retriever(u8, default = 0)
    reversed: int = Retriever(u8, default = 0)
    """unused"""

    def __new__(
        cls,
        ver: Version = DE_LATEST,
        init_defaults = True,
        **retriever_inits
    ):
        return super().__new__(cls, ver, ctx or Context(), init_defaults, **retriever_inits)
