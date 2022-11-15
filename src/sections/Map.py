from __future__ import annotations

from src.retrievers.Retriever import Retriever
from src.types.BaseStruct import BaseStruct
from src.types.Bool import Bool8
from src.types.Bytes import Bytes
from src.types.Int import UInt32, UInt8, Int16
from src.types.Str import Str16


class Terrain(BaseStruct):
    terrain_id: int = Retriever(UInt8, default = 0)
    elevation: int = Retriever(UInt8, default = 0)
    unused: bytes = Retriever(Bytes[3], default = b"\x00\xff\xff")
    layer: int = Retriever(Int16, default = -1)


class Map(BaseStruct):
    @staticmethod
    def set_terrain_data_repeat(retriever: Retriever, instance: Map):
        Map.tiles.set_repeat(instance, instance.width * instance.height)  # type: ignore

    string_starter1: bytes = Retriever(Bytes[2], default = b"\x60\x0a")
    water_definition: str = Retriever(Str16, default = "")
    string_starter2: bytes = Retriever(Bytes[2], default = b"\x60\x0a")
    colour_mood: str = Retriever(Str16, default = "Empty")
    string_starter3: bytes = Retriever(Bytes[2], default = b"\x60\x0a")
    script_name: str = Retriever(Str16, default = "")                              # todo: dep
    collide_and_correct: bool = Retriever(Bool8, default = False)
    villager_force_drop: bool = Retriever(Bool8, default = False)
    unknown: bytes = Retriever(Bytes[128], default = b"\xff"*128)
    lock_coop_alliances: bool = Retriever(Bool8, default = False)
    population_caps: list[int] = Retriever(UInt32, default = 200, repeat = 16)
    unknown3 = Retriever(Bytes[4], default = b"\x00"*4)
    unknown5 = Retriever(Bytes[4], default = b"\x0d\xf0\xad\xde")
    unknown4 = Retriever(Bytes[4], default = b"\x02"+b"\x00"*3)
    no_waves_on_shore: bool = Retriever(Bool8, default = False)
    width: int = Retriever(UInt32, default = 120, on_set = [set_terrain_data_repeat]) # type: ignore
    height: int = Retriever(UInt32, default = 120, on_set = [set_terrain_data_repeat]) # type: ignore
    tiles: list[Terrain] = Retriever(Terrain, default = Terrain())

    def __init__(self, struct_version: tuple[int, ...] = (1, 47)):
        super().__init__(struct_version)
