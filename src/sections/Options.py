from src.retrievers.Retriever import Retriever
from src.types.Array import StackedArray32s
from src.types.BaseStruct import BaseStruct
from src.types.Bool import Bool32
from src.types.Bytes import Bytes
from src.types.Int import UInt32, UInt8


class Options(BaseStruct):
    disabled_tech_ids: list[list[int]] = Retriever(StackedArray32s[UInt32, 16], default = [[] for _ in range(16)])
    disabled_unit_ids: list[list[int]] = Retriever(StackedArray32s[UInt32, 16], default = [[] for _ in range(16)])
    disabled_building_ids: list[list[int]] = Retriever(StackedArray32s[UInt32, 16], default = [[] for _ in range(16)])
    combat_mode: bool = Retriever(Bool32, default = False)
    naval_mode: bool = Retriever(Bool32, default = False)
    all_techs: bool = Retriever(Bool32, default = False)
    starting_ages: list[int] = Retriever(UInt32, default = 2, repeat = 16)
    unknown1: bytes = Retriever(Bytes[12], default = b"\x9d"+b"\xff"*11)
    ai_map_type: int = Retriever(UInt32, default = 0)
    unknown3: bytes = Retriever(Bytes[1], default = b"\x00")
    base_priorities: list[int] = Retriever(UInt8, default = 0, repeat = 8)
    unknown2: bytes = Retriever(Bytes[7], default = b"\x00"*7)
    num_triggers: int = Retriever(UInt32, default = 0) # todo: dep

    def __init__(self, struct_version: tuple[int, ...] = (1, 47)):
        super().__init__(struct_version)
