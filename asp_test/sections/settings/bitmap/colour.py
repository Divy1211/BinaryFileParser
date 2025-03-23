from bfp_rs import BaseStruct, Retriever, Version
from bfp_rs.types.le import u8

from asp_test.sections.scx_versions import DE_LATEST


class Colour(BaseStruct):
    red: int   = Retriever(u8, default = 0)
    green: int = Retriever(u8, default = 0)
    blue: int  = Retriever(u8, default = 0)
    alpha: int = Retriever(u8, default = 0)
    """unused"""

    def __new__(
        cls,
        ver: Version = DE_LATEST,
        init_defaults = True,
        **retriever_inits
    ):
        return super().__new__(cls, ver, init_defaults, **retriever_inits)
