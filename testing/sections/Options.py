from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import bool32, Bytes, StackedArray32s, uint32, uint8
from testing.sections.MapData import View


class Options(BaseStruct):
    @staticmethod
    def update_num_triggers(retriever: Retriever, instance: Options):
        instance.num_triggers = len(instance.parent.trigger_data.triggers)

    disabled_tech_ids: list[list[int]] = Retriever(StackedArray32s[uint32, 16], default_factory = lambda _, __: [[] for _ in range(16)])
    disabled_unit_ids: list[list[int]] = Retriever(StackedArray32s[uint32, 16], default_factory = lambda _, __: [[] for _ in range(16)])
    disabled_building_ids: list[list[int]] = Retriever(StackedArray32s[uint32, 16], default_factory = lambda _, __: [[] for _ in range(16)])
    combat_mode: bool = Retriever(bool32, default = False)
    naval_mode: bool = Retriever(bool32, default = False)
    all_techs: bool = Retriever(bool32, default = False)
    starting_ages: list[int] = Retriever(uint32, default = 2, repeat = 16)
    separator: bytes = Retriever(Bytes[4], default = b"\x9d\xff\xff\xff")
    player1_view: View = Retriever(View, default_factory = lambda sv, p: View(sv, p))
    ai_map_type_unused_1_45: int = Retriever(uint32, default = 2, max_ver = Version((1, 45)))
    ai_map_type_unused_1_46: int = Retriever(uint32, default = 22, min_ver = Version((1, 46)), max_ver = Version((1, 46)))
    ai_map_type: int = Retriever(uint32, default = 0, min_ver = Version((1, 47)))
    unknown3: bytes = Retriever(Bytes[1], default = b"\x00")
    base_priorities: list[int] = Retriever(uint8, default = 0, repeat = 8)
    unknown2: bytes = Retriever(Bytes[7], default = b"\x00"*7)
    num_triggers: int = Retriever(uint32, default = 0, on_write = [update_num_triggers])

    def __init__(self, struct_ver: Version = Version((1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, parent, initialise_defaults = initialise_defaults, **retriever_inits)
