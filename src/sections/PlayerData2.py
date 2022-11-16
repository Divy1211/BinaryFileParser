from __future__ import annotations

from src.retrievers.Retriever import Retriever
from src.types.BaseStruct import BaseStruct
from src.types.Bytes import Bytes
from src.types.Int import UInt8, UInt32, Int32
from src.types.Str import Str16, Str32


class AiFile(BaseStruct):
    unknown: bytes = Retriever(Bytes[8], default = b"\x00"*8)
    per_content: list[str] = Retriever(Str32, default = "")

    def __init__(self, struct_version: tuple[int, ...] = (1, 47)):
        super().__init__(struct_version)


class Resources(BaseStruct):
    @staticmethod
    def update_player_data4_gold(retriever: Retriever, instance: Resources):
        for i in range(8):
            instance.parent.parent.unit_data.player_data4[i].gold = instance.parent.resources[i].gold

    @staticmethod
    def update_player_data4_wood(retriever: Retriever, instance: Resources):
        for i in range(8):
            instance.parent.parent.unit_data.player_data4[i].gold = instance.parent.resources[i].wood

    @staticmethod
    def update_player_data4_food(retriever: Retriever, instance: Resources):
        for i in range(8):
            instance.parent.parent.unit_data.player_data4[i].gold = instance.parent.resources[i].food

    @staticmethod
    def update_player_data4_stone(retriever: Retriever, instance: Resources):
        for i in range(8):
            instance.parent.parent.unit_data.player_data4[i].gold = instance.parent.resources[i].stone

    @staticmethod
    def update_player_data4_ore_x(retriever: Retriever, instance: Resources):
        for i in range(8):
            instance.parent.parent.unit_data.player_data4[i].gold = instance.parent.resources[i].ore_x

    @staticmethod
    def update_player_data4_trade_goods(retriever: Retriever, instance: Resources):
        for i in range(8):
            instance.parent.parent.unit_data.player_data4[i].gold = instance.parent.resources[i].trade_goods

    gold: int = Retriever(Int32, default = 0, on_write = [update_player_data4_gold])
    wood: int = Retriever(Int32, default = 0, on_write = [update_player_data4_wood])
    food: int = Retriever(Int32, default = 0, on_write = [update_player_data4_food])
    stone: int = Retriever(Int32, default = 0, on_write = [update_player_data4_stone])
    ore_x: int = Retriever(Int32, default = 0, on_write = [update_player_data4_ore_x])
    """unused"""
    trade_goods: int = Retriever(Int32, default = 0, on_write = [update_player_data4_trade_goods])
    """unused"""
    player_colour: int = Retriever(Int32, default = 0)

    def __init__(self, struct_version: tuple[int, ...] = (1, 47)):
        super().__init__(struct_version)


class PlayerData2(BaseStruct):
    strings: list[str] = Retriever(Str16, default = "", repeat = 32)
    ai_names: list[str] = Retriever(Str16, default = "", repeat = 16)
    ai_files: list[AiFile] = Retriever(AiFile, default = AiFile(), repeat = 16)
    ai_types: list[int] = Retriever(UInt8, default = 1, repeat = 16)
    separator: int = Retriever(UInt32, default = 4294967197)
    resources: list[Resources] = Retriever(Resources, default = Resources(), repeat = 16)

    def __init__(self, struct_version: tuple[int, ...] = (1, 47)):
        super().__init__(struct_version)
