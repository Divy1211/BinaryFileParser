from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import bool32, Bytes, FixedLenStr, float32, str16, uint32


class PlayerData1(BaseStruct):
    active: bool = Retriever(bool32, default = False)
    human: bool = Retriever(bool32, default = False)
    civilization_1_36: int = Retriever(uint32, default = 36, min_ver = Version((1, 36)), max_ver = Version((1, 40)))
    architecture_set_1_40: int = Retriever(uint32, default = 36, min_ver = Version((1, 40)), max_ver = Version((1, 40)))
    civilization_1_41: int = Retriever(uint32, default = 38, min_ver = Version((1, 41)), max_ver = Version((1, 42)))
    architecture_set_1_41: int = Retriever(uint32, default = 38, min_ver = Version((1, 41)), max_ver = Version((1, 42)))
    civilization_1_43: int = Retriever(uint32, default = 40, min_ver = Version((1, 43)), max_ver = Version((1, 45)))
    architecture_set_1_43: int = Retriever(uint32, default = 40, min_ver = Version((1, 43)), max_ver = Version((1, 45)))
    civilization_1_46: int = Retriever(uint32, default = 43, min_ver = Version((1, 46)))
    architecture_set_1_46: int = Retriever(uint32, default = 43, min_ver = Version((1, 46)))
    cty_mode: int = Retriever(uint32, default = 4)

    def __init__(self, struct_ver: Version = Version((1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, parent, initialise_defaults = initialise_defaults, **retriever_inits)


class DataHeader(BaseStruct):
    next_unit_id: int = Retriever(uint32, default = 0)
    version: float = Retriever(float32, default = 1.4700000286102295)
    tribe_names: list[str] = Retriever(FixedLenStr[256], default = "0"*256, repeat = 16)
    player_name_str_ids: list[int] = Retriever(uint32, default = 4294967294, repeat = 16)
    player_data1: list[PlayerData1] = Retriever(PlayerData1, default = PlayerData1(), repeat = 16)
    lock_civilizations: list[bool] = Retriever(bool32, default = False, repeat = 16)
    unknown: bytes = Retriever(Bytes[9], default = b"\x00"+b"\x00"*8, max_ver = Version((1, 45)))
    unknown_1_46: bytes = Retriever(Bytes[9], default = b"\x01"+b"\x00"*8, min_ver = Version((1, 46)))
    file_name: str = Retriever(str16, default = "MadeWithAoE2SP.aoe2scenario")

    def __init__(self, struct_ver: Version = Version((1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, parent, initialise_defaults = initialise_defaults, **retriever_inits)
