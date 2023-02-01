from __future__ import annotations

from binary_file_parser import Retriever, BaseStruct
from binary_file_parser.types import bool32, Bytes, uint32, uint8, StackedArray32s, FixedLenArray, float32


class LegacyDisables(BaseStruct):
    num_disabled_techs: list[int] = Retriever(FixedLenArray[uint32, 16], default = [0] * 16)
    disabled_tech_ids1: list[list[int]] = Retriever(FixedLenArray[uint32, 30], default = [[] for _ in range(16)], repeat = 16)
    disabled_tech_ids2: list[list[int]] = Retriever(FixedLenArray[uint32, 30], default = [[] for _ in range(16)], repeat = 16, min_ver = (1, 30))

    num_disabled_units: list[int] = Retriever(FixedLenArray[uint32, 16], default = [0] * 16)
    disabled_unit_ids1: list[list[int]] = Retriever(FixedLenArray[uint32, 30], default = [[] for _ in range(16)], repeat = 16)
    disabled_unit_ids2: list[list[int]] = Retriever(FixedLenArray[uint32, 30], default = [[] for _ in range(16)], repeat = 16, min_ver = (1, 30))

    num_disabled_buildings: list[int] = Retriever(FixedLenArray[uint32, 16], default = [0] * 16)
    disabled_building_ids1: list[list[int]] = Retriever(FixedLenArray[uint32, 30], default = [[] for _ in range(16)], repeat = 16)
    disabled_building_ids2: list[list[int]] = Retriever(FixedLenArray[uint32, 30], default = [[] for _ in range(16)], repeat = 16, min_ver = (1, 30))

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True):
        super().__init__(struct_version, parent, initialise_defaults)


class Options(BaseStruct):
    @staticmethod
    def update_num_triggers(retriever: Retriever, instance: Options):
        instance.num_triggers = len(instance.parent.trigger_data.triggers)

    legacy_disables: LegacyDisables = Retriever(LegacyDisables, default = LegacyDisables(), max_ver = (1, 30))
    disabled_tech_ids: list[list[int]] = Retriever(StackedArray32s[uint32, 16], default = [[] for _ in range(16)], min_ver = (1, 36))
    disabled_unit_ids: list[list[int]] = Retriever(StackedArray32s[uint32, 16], default = [[] for _ in range(16)], min_ver = (1, 36))
    disabled_building_ids: list[list[int]] = Retriever(StackedArray32s[uint32, 16], default = [[] for _ in range(16)], min_ver = (1, 36))
    combat_mode: bool = Retriever(bool32, default = False)
    naval_mode: bool = Retriever(bool32, default = False)
    all_techs: bool = Retriever(bool32, default = False)
    starting_ages: list[int] = Retriever(uint32, default = 2, repeat = 16)
    separator: bytes = Retriever(Bytes[4], default = b"\x9d\xff\xff\xff")
    editor_camera_x: int = Retriever(uint32, default = 0)
    editor_camera_y: int = Retriever(uint32, default = 0)
    # unknown1_1_21: bytes = Retriever(Bytes[8], default = b"\x07\x00\x00\x00\xbf\x00\x00\x00", max_ver = (1, 21))
    # unknown1_1_45: bytes = Retriever(Bytes[8], default = b"u\x00\x00\x00.\x00\x00\x00", min_ver = (1, 36), max_ver = (1, 45))
    # unknown1_1_46: bytes = Retriever(Bytes[8], default = b"\xff"*8, min_ver = (1, 46))
    ai_map_type_unused_1_36: int = Retriever(uint32, default = 2, min_ver = (1, 36), max_ver = (1, 45))
    ai_map_type_unused_1_46: int = Retriever(uint32, default = 22, min_ver = (1, 46), max_ver = (1, 46))
    ai_map_type: int = Retriever(uint32, default = 0, min_ver = (1, 47))
    unknown4: bytes = Retriever(Bytes[4], default = b"\x00"*3, max_ver = (1, 21))
    unknown3: bytes = Retriever(Bytes[1], default = b"\x00")
    base_priorities: list[int] = Retriever(uint8, default = 0, repeat = 8)

    unknown2: bytes = Retriever(Bytes[7], default = b"\x00"*7, min_ver = (1, 36))
    num_triggers: int = Retriever(uint32, default = 0, on_write = [update_num_triggers], min_ver = (1, 36))

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True):
        super().__init__(struct_version, parent, initialise_defaults)
