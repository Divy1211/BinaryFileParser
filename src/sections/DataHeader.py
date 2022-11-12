from __future__ import annotations

import operator
from functools import partial

from src.retrievers.Retriever import Retriever
from src.types.BaseStruct import BaseStruct
from src.types.Float import Float32
from src.types.Int import UInt32
from src.types.Str import FixedLenStr, each_len


class DataHeader(BaseStruct):
    next_unit_id: int = Retriever(UInt32, default = 0)
    version: float = Retriever(Float32, default = 1.42)
    tribe_names: list[str] = Retriever(FixedLenStr(256), default = "0"*256, repeat = 16, validators = [partial(each_len, operator.eq, 256)])
    string_table_player_names: list[int] = Retriever(UInt32, default = 4294967294, repeat = 16)

    def __init__(self, version: tuple[int, ...] = (1, 47)):
        super().__init__(version)
