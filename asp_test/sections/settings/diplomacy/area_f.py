from bfp_rs import BaseStruct, Retriever, Version, Context

from asp_test.sections.settings.diplomacy.tile_f import TileF
from asp_test.sections.scx_versions import DE_LATEST


class AreaF(BaseStruct):
    corner1: TileF = Retriever(TileF, default_factory = TileF)
    corner2: TileF = Retriever(TileF, default_factory = TileF)

    def __new__(cls, ver: Version = DE_LATEST, ctx: Context = None, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, ctx or Context(), init_defaults, **retriever_inits)
