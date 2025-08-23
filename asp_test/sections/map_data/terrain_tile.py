from __future__ import annotations

from bfp_rs import BaseStruct, Retriever, Version, Context
from bfp_rs.types.le import i16, i8, u8

from asp_test.sections.scx_versions import MAP_LATEST

class TerrainTile(BaseStruct):
    # @formatter:off
    type: int       = Retriever(u8,                        default = 0)
    elevation: int  = Retriever(i8,                        default = 0)
    zone: int       = Retriever(i8,                        default = 0)
    """unused?"""
    mask_type: int  = Retriever(i16, min_ver = Version(1), default = -1)
    """what does this do?"""
    layer_type: int = Retriever(i16, min_ver = Version(1), default = -1)
    # @formatter:on

    def __new__(cls, ver: Version = MAP_LATEST, ctx: Context = None, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, ctx or Context(), init_defaults, **retriever_inits)
