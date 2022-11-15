from __future__ import annotations

from src.retrievers.Retriever import Retriever
from src.types.Array import Array16, FixedLenArray, Array32
from src.types.BaseStruct import BaseStruct
from src.types.Bool import Bool8
from src.types.Bytes import Bytes
from src.types.Float import Float32
from src.types.Int import UInt32, UInt8, Int16, UInt16, Int32
from src.types.Str import NullTermStr16


class PlayerData4(BaseStruct):
    food: float = Retriever(Float32, default = 0.0)
    """duplicate"""
    wood: float = Retriever(Float32, default = 0.0)
    """duplicate"""
    gold: float = Retriever(Float32, default = 0.0)
    """duplicate"""
    stone: float = Retriever(Float32, default = 0.0)
    """duplicate"""
    ore_x: float = Retriever(Float32, default = 0.0)
    """duplicate"""
    trade_goods: float = Retriever(Float32, default = 0.0)
    population_limit: float = Retriever(Float32, default = 200.0)

    def __init__(self, struct_version: tuple[int, ...] = (1, 47)):
        super().__init__(struct_version)


class PlayerData3(BaseStruct):
    @staticmethod
    def set_unknown2_repeat(retriever: Retriever, instance: PlayerData3):
        PlayerData3.unknown2.set_repeat(instance, 7 if instance.victory_version == 2.0 else -1)

    @staticmethod
    def set_num_wwc2_repeat(retriever: Retriever, instance: PlayerData3):
        if instance.victory_version != 2.0:
            PlayerData3.num_ww_campaign2.set_repeat(instance, -1)

    @staticmethod
    def set_gte_repeat(retriever: Retriever, instance: PlayerData3):
        PlayerData3.grand_theft_empires.set_repeat(instance, instance.num_grand_theft_empires)

    @staticmethod
    def set_wwc2_repeat(retriever: Retriever, instance: PlayerData3):
        PlayerData3.ww_campaign2.set_repeat(instance, instance.num_ww_campaign2)

    @staticmethod
    def update_num_gte(retriever: Retriever, instance: PlayerData3):
        instance.num_grand_theft_empires = len(instance.grand_theft_empires)

    @staticmethod
    def update_num_wwc2(retriever: Retriever, instance: PlayerData3):
        instance.num_ww_campaign2 = len(instance.ww_campaign2)

    constant_name: str = Retriever(NullTermStr16, default = "Scenario Editor Phantom")
    editor_camera_x: float = Retriever(Float32, default = 72.0)
    editor_camera_y: float = Retriever(Float32, default = 72.0)
    initial_camera_x: int = Retriever(Int16, default = 72)
    initial_camera_y: int = Retriever(Int16, default = 72)
    aok_allied_victory: bool = Retriever(Bool8, default = False)
    diplomacy_stances_interaction: list[int] = Retriever(Array16[UInt8], default = [3, 0, 3, 3, 3, 3, 3, 3, 3])
    diplomacy_stances_ai_system: list[int] = Retriever(FixedLenArray[9, UInt32], default = [0, 1, 4, 4, 4, 4, 4, 4, 4])
    colour: int = Retriever(UInt32, default = 0)
    victory_version: float = Retriever(Float32, default = 2.0, on_set = [set_unknown2_repeat, set_num_wwc2_repeat])
    num_grand_theft_empires: int = Retriever(UInt16, default = 0, on_set = [set_gte_repeat], on_write = [update_num_gte])
    unknown2: list[int] = Retriever(UInt8, default = 0)
    grand_theft_empires: list[bytes] = Retriever(Bytes[44], default = b"\x00"*44)
    """unknown structure"""
    num_ww_campaign2: int = Retriever(UInt8, default = 0, on_set = [set_wwc2_repeat], on_write = [update_num_wwc2])
    unknown3: int = Retriever(UInt8, default = 0, repeat = 7)
    ww_campaign2: list[bytes] = Retriever(Bytes[32], default = [])
    """unknown structure"""
    unknown4: int = Retriever(Int32, default = -1)

    def __init__(self, struct_version: tuple[int, ...] = (1, 47)):
        super().__init__(struct_version)


class Unit(BaseStruct):
    x: float = Retriever(Float32, default = 0.5)
    y: float = Retriever(Float32, default = 0.5)
    z: float = Retriever(Float32, default = 0)
    reference_id: int = Retriever(Int32, default = 0)
    const: int = Retriever(UInt16, default = 4)
    status: int = Retriever(UInt8, default = 2)
    rotation: float = Retriever(Float32, default = 0)
    initial_animation_frame: int = Retriever(UInt16, default = 0)
    garrisoned_in_reference_id: int = Retriever(Int32, default = -1)

    def __init__(self, struct_version: tuple[int, ...] = (1, 47)):
        super().__init__(struct_version)


class UnitData(BaseStruct):
    @staticmethod
    def set_units_repeat(retriever: Retriever, instance: UnitData):
        UnitData.units.set_repeat(instance, instance.num_unit_lists)

    @staticmethod
    def update_num_unit_lists(retriever: Retriever, instance: UnitData):
        instance.num_unit_lists = len(instance.units)
        instance.num_players = len(instance.units)

    num_unit_lists: int = Retriever(UInt32, default = 9, on_set = [set_units_repeat], on_write = [update_num_unit_lists])
    player_data4: list[PlayerData4] = Retriever(PlayerData4, default = PlayerData4(), repeat = 8)
    num_players: int = Retriever(UInt32, default = 9)
    player_data3: list[PlayerData3] = Retriever(PlayerData3, default = PlayerData3(), repeat = 8)
    units: list[list[Unit]] = Retriever(Array32[Unit], default = [])

    def __init__(self, struct_version: tuple[int, ...] = (1, 47)):
        super().__init__(struct_version)
