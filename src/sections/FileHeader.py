from __future__ import annotations

from src.retrievers.Retriever import Retriever
from src.types.BaseStruct import BaseStruct
from src.types.Array import Array32
from src.types.Int import uint32, int32
from src.types.Str import FixedLenStr, nt_str32


class FileHeader(BaseStruct):
    @staticmethod
    def update_num_triggers(retriever: Retriever, instance: FileHeader):
        instance.num_triggers = len(instance.parent.trigger_data.triggers)

    file_version: str = Retriever(FixedLenStr[4], default = "1.47")
    header_len: int = Retriever(uint32, default = 0)
    savable: int = Retriever(int32, default = 6)
    timestamp_of_last_save: int = Retriever(uint32, default = 1610675127)
    scenario_instructions: str = Retriever(nt_str32, default = "")
    num_players: int = Retriever(uint32, default = 2)
    unknown1: int = Retriever(uint32, default = 1000)
    """always (?) 1k"""
    unknown2: int = Retriever(uint32, default = 1)
    """always (?) 1"""
    unknowns: list[int] = Retriever(Array32[uint32], default = [2, 3, 4, 5, 6, 7])
    creator: str = Retriever(nt_str32, default = "AoE2ScenarioParser")
    num_triggers: int = Retriever(uint32, default = 0, on_write = [update_num_triggers])

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True):
        super().__init__(struct_version, parent, initialise_defaults)
