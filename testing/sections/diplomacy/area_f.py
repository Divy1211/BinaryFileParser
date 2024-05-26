from binary_file_parser import BaseStruct, Retriever, Version
from testing.sections.diplomacy.tile_f import TileF
from testing.sections.scx_versions import DE_LATEST


class AreaF(BaseStruct):
    corner1: TileF = Retriever(TileF, default_factory = lambda sv: TileF(sv))
    corner2: TileF = Retriever(TileF, default_factory = lambda sv: TileF(sv))

    def __init__(self, struct_ver: Version = DE_LATEST, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
