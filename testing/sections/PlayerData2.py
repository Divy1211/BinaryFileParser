from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import Bytes, int32, str16, str32, uint32, uint8


class AiFile(BaseStruct):
    unknown: bytes = Retriever(Bytes[8], default = b"\x00"*8)
    per_content: list[str] = Retriever(str32, default = "")

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)


class Resources(BaseStruct):
    # todo: corresponding index to be accessed/updated for individual structs in list
    gold: int = Retriever(int32, default = 0)
    wood: int = Retriever(int32, default = 0)
    food: int = Retriever(int32, default = 0)
    stone: int = Retriever(int32, default = 0)
    ore_x: int = Retriever(int32, default = 0)
    """unused"""
    trade_goods: int = Retriever(int32, default = 0)
    """unused"""
    player_colour: int = Retriever(int32, default = 0)

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)


class PlayerData2(BaseStruct):
    strings: list[str] = Retriever(str16, default = "", repeat = 32)
    ai_names: list[str] = Retriever(str16, default = "", repeat = 16)
    ai_files: list[AiFile] = Retriever(AiFile, default_factory = lambda sv, p: AiFile(sv, p), repeat = 16)
    ai_types: list[int] = Retriever(uint8, default = 1, repeat = 16)
    separator: int = Retriever(uint32, default = 4294967197)
    resources: list[Resources] = Retriever(Resources, default_factory = lambda sv, p: Resources(sv, p), repeat = 16)

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
