from __future__ import annotations

from src import Retriever
from binary_file_parser.retrievers.BaseStruct import BaseStruct
from binary_file_parser.types.Bytes import Bytes
from binary_file_parser.types.Int import uint8, uint32, int32
from binary_file_parser.types.Str import str16, str32


class AiFile(BaseStruct):
    unknown: bytes = Retriever(Bytes[8], default = b"\x00"*8)
    per_content: list[str] = Retriever(str32, default = "")

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True):
        super().__init__(struct_version, parent, initialise_defaults)


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

    gold: int = Retriever(int32, default = 0, on_write = [update_player_data4_gold])
    wood: int = Retriever(int32, default = 0, on_write = [update_player_data4_wood])
    food: int = Retriever(int32, default = 0, on_write = [update_player_data4_food])
    stone: int = Retriever(int32, default = 0, on_write = [update_player_data4_stone])
    ore_x: int = Retriever(int32, default = 0, on_write = [update_player_data4_ore_x])
    """unused"""
    trade_goods: int = Retriever(int32, default = 0, on_write = [update_player_data4_trade_goods])
    """unused"""
    player_colour: int = Retriever(int32, default = 0)

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True):
        super().__init__(struct_version, parent, initialise_defaults)


class PlayerData2(BaseStruct):
    strings: list[str] = Retriever(str16, default = "", repeat = 32)
    ai_names: list[str] = Retriever(str16, default = "", repeat = 16)
    ai_files: list[AiFile] = Retriever(AiFile, default = AiFile(), repeat = 16)
    ai_types: list[int] = Retriever(uint8, default = 1, repeat = 16)
    separator: int = Retriever(uint32, default = 4294967197)
    resources: list[Resources] = Retriever(Resources, default = Resources(), repeat = 16)

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True):
        super().__init__(struct_version, parent, initialise_defaults)
