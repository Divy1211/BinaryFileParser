from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever
from binary_file_parser.types import Bytes, int32, str16, str32, uint32, uint8


class AiFile(BaseStruct):
    unknown: bytes = Retriever(Bytes[8], default = b"\x00"*8)
    per_content: list[str] = Retriever(str32, default = "")

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_version, parent, initialise_defaults, **retriever_inits)


class Resources(BaseStruct):
    # todo: parents on list structs
    # todo: corresponding index to be accessed/updated for individual structs in list
    @staticmethod
    def update_player_data4_gold(retriever: Retriever, instance: Resources):
        for i in range(8):
            instance.parent.parent.unit_data.player_data4[i].gold = instance.parent.resources[i].gold

    @staticmethod
    def update_player_data4_wood(retriever: Retriever, instance: Resources):
        for i in range(8):
            instance.parent.parent.unit_data.player_data4[i].wood = instance.parent.resources[i].wood

    @staticmethod
    def update_player_data4_food(retriever: Retriever, instance: Resources):
        for i in range(8):
            instance.parent.parent.unit_data.player_data4[i].food = instance.parent.resources[i].food

    @staticmethod
    def update_player_data4_stone(retriever: Retriever, instance: Resources):
        for i in range(8):
            instance.parent.parent.unit_data.player_data4[i].stone = instance.parent.resources[i].stone

    @staticmethod
    def update_player_data4_ore_x(retriever: Retriever, instance: Resources):
        for i in range(8):
            instance.parent.parent.unit_data.player_data4[i].ore_x = instance.parent.resources[i].ore_x

    @staticmethod
    def update_player_data4_trade_goods(retriever: Retriever, instance: Resources):
        for i in range(8):
            instance.parent.parent.unit_data.player_data4[i].trade_goods = instance.parent.resources[i].trade_goods

    gold: int = Retriever(int32, default = 0, on_write = [update_player_data4_gold])
    wood: int = Retriever(int32, default = 0, on_write = [update_player_data4_wood])
    food: int = Retriever(int32, default = 0, on_write = [update_player_data4_food])
    stone: int = Retriever(int32, default = 0, on_write = [update_player_data4_stone])
    ore_x: int = Retriever(int32, default = 0, on_write = [update_player_data4_ore_x])
    """unused"""
    trade_goods: int = Retriever(int32, default = 0, on_write = [update_player_data4_trade_goods])
    """unused"""
    player_colour: int = Retriever(int32, default = 0)

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_version, parent, initialise_defaults, **retriever_inits)


class PlayerData2(BaseStruct):
    strings: list[str] = Retriever(str16, default = "", repeat = 32)
    ai_names: list[str] = Retriever(str16, default = "", repeat = 16)
    ai_files: list[AiFile] = Retriever(AiFile, default = AiFile(), repeat = 16)
    ai_types: list[int] = Retriever(uint8, default = 1, repeat = 16)
    separator: int = Retriever(uint32, default = 4294967197)
    resources: list[Resources] = Retriever(Resources, default = Resources(), repeat = 16)

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_version, parent, initialise_defaults, **retriever_inits)
