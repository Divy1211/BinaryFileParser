from __future__ import annotations

import operator
from functools import partial

from src.retrievers.Retriever import Retriever
from src.types.BaseStruct import BaseStruct
from src.types.Bool import Bool32
from src.types.Bytes import Bytes
from src.types.Float import Float32
from src.types.Int import UInt32
from src.types.Str import FixedLenStr, each_len, Str16


class PlayerData1(BaseStruct):
    active: int = Retriever(UInt32, default = 0)
    human: int = Retriever(UInt32, default = 0)
    civilization: int = Retriever(UInt32, default = 43)
    architecture_set: int = Retriever(UInt32, default = 43)
    cty_mode: int = Retriever(UInt32, default = 4)


class DataHeader(BaseStruct):
    next_unit_id: int = Retriever(UInt32, default = 0)
    version: float = Retriever(Float32, default = 1.42)
    tribe_names: list[str] = Retriever(FixedLenStr(256), default = "0"*256, repeat = 16, validators = [partial(each_len, operator.eq, 256)])
    string_table_player_names: list[int] = Retriever(UInt32, default = 4294967294, repeat = 16)
    player_data_1: list[PlayerData1] = Retriever(PlayerData1, default = PlayerData1(), repeat = 16)
    lock_civs: list[int] = Retriever(Bool32, default = False, repeat = 16)
    unknown: bytes = Retriever(Bytes(9), default = b"\x01"+b"\x00"*7)
    filename: str = Retriever(Str16, default = "MadeWithAoE2SP.aoe2scenario")

    def __init__(self, version: tuple[int, ...] = (1, 47)):
        super().__init__(version)
