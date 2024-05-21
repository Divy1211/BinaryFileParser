from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import bool32, float32, int32, uint32


class LegacyVictoryInfo(BaseStruct):
    # @formatter:off
    object_type: int =            Retriever(int32,   default = 0)
    all: bool =                   Retriever(bool32,  default = 0)
    player_id: int =              Retriever(int32,   default = 0)
    destination_object_id: int =  Retriever(int32,   default = 0)
    area_x1: float =              Retriever(float32, default = 0)
    area_y1: float =              Retriever(float32, default = 0)
    area_x2: float =              Retriever(float32, default = 0)
    area_y2: float =              Retriever(float32, default = 0)
    victory_type: int =           Retriever(int32,   default = 0)
    amount: int =                 Retriever(int32,   default = 0)
    attribute: int =              Retriever(int32,   default = 0)
    object_id: int =              Retriever(int32,   default = 0)
    destination_object_id2: int = Retriever(int32,   default = 0)
    object: int =                 Retriever(uint32,  default = 0)
    destination_object: int =     Retriever(uint32,  default = 0)
    # @formatter:on

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
