from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import bool8, Bytes, int16, int32, str16, uint32, uint8


class View(BaseStruct):
    x: int = Retriever(int32, default = -1)
    y: int = Retriever(int32, default = -1)

    def __init__(self, struct_version: Version = Version((1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_version, parent, initialise_defaults, **retriever_inits)


class Terrain(BaseStruct):
    terrain_id: int = Retriever(uint8, default = 0)
    elevation: int = Retriever(uint8, default = 0)
    unused: bytes = Retriever(Bytes[3], default = b"\x00\xff\xff")
    layer: int = Retriever(int16, default = -1)

    def __init__(self, struct_version: Version = Version((1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_version, parent, initialise_defaults, **retriever_inits)


class MapData(BaseStruct):
    @staticmethod
    def set_terrain_data_repeat(retriever: Retriever, instance: MapData):
        MapData.tiles.set_repeat(instance, instance.width * instance.height)

    @staticmethod
    def update_script_file_path(retriever: Retriever, instance: MapData):
        instance.parent.file_data.script_file_path = instance.script_name+".xs" if instance.script_name else ""

    @staticmethod
    def update_width_height(retriever: Retriever, instance: MapData):
        instance.width = instance.height = int(len(instance.tiles) ** 0.5)

    string_starter1: bytes = Retriever(Bytes[2], default = b"\x60\x0a")
    water_definition: str = Retriever(str16, default = "")
    string_starter2: bytes = Retriever(Bytes[2], default = b"\x60\x0a")
    colour_mood: str = Retriever(str16, default = "Empty")
    string_starter3: bytes = Retriever(Bytes[2], default = b"\x60\x0a", min_ver = Version((1, 40)))
    script_name: str = Retriever(str16, default = "", on_write = [update_script_file_path], min_ver = Version((1, 40)))
    lock_coop_alliances_1_41: bool = Retriever(bool8, default = False, min_ver = Version((1, 41)), max_ver = Version((1, 41)))
    collide_and_correct: bool = Retriever(bool8, default = False)
    villager_force_drop: bool = Retriever(bool8, default = False, min_ver = Version((1, 37)))
    player_views: list[View] = Retriever(View, default = View(), min_ver = Version((1, 40)), repeat = 16)
    lock_coop_alliances_1_42: bool = Retriever(bool8, default = False, min_ver = Version((1, 42)))
    ai_map_type: int = Retriever(uint32, default = 0, min_ver = Version((1, 42)), max_ver = Version((1, 46)))
    population_caps: list[int] = Retriever(uint32, default = 200, repeat = 16, min_ver = Version((1, 44)))
    secondary_game_mode = Retriever(Bytes[4], default = b"\x00"*4, min_ver = Version((1, 45)))
    unknown3 = Retriever(Bytes[4], default = b"\x0d\xf0\xad\xde")
    unknown4 = Retriever(Bytes[4], default = b"\x02"+b"\x00"*3)
    no_waves_on_shore: bool = Retriever(bool8, default = False)
    width: int = Retriever(uint32, default = 120, on_set = [set_terrain_data_repeat], on_write = [update_width_height])
    height: int = Retriever(uint32, default = 120, on_set = [set_terrain_data_repeat])
    tiles: list[Terrain] = Retriever(Terrain, default = Terrain(), repeat = 14_400)

    def __init__(self, struct_version: Version = Version((1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_version, parent, initialise_defaults, **retriever_inits)
