from __future__ import annotations

from functools import partial

from src.retrievers.Retriever import Retriever
from src.types.BaseStruct import BaseStruct
from src.types.Array import Array
from src.types.Int import UInt32, Int32
from src.types.Str import FixedLenStr, chk_len, NullTermStr32


class FileHeader(BaseStruct):
    file_version_str: str = Retriever(FixedLenStr(4), default = "1.47", validators = [partial(chk_len, 4)])
    header_len: int = Retriever(UInt32, default = 0)
    savable: int = Retriever(Int32, default = 6)
    timestamp_of_last_save: int = Retriever(UInt32, default = 1610675127)
    scenario_instructions: str = Retriever(NullTermStr32, default = "")
    num_players: int = Retriever(UInt32, default = 2)
    unknown1: int = Retriever(UInt32, default = 1000)
    unknown2: int = Retriever(UInt32, default = 1)
    unknowns: list[int] = Retriever(Array(UInt32), default = [2, 3, 4, 5, 6, 7])
    creator: str = Retriever(NullTermStr32, default = "Alian713")
    num_triggers: int = Retriever(UInt32, default = 420)

    def __init__(self, version: tuple[int, ...] = (1, 47)):
        super().__init__(version)
