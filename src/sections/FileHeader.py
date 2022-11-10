from __future__ import annotations

from functools import partial
from typing import TYPE_CHECKING

from src.retrievers.Retriever import Retriever
from src.sections.BaseSection import BaseSection
from src.types.Array import Array
from src.types.Int import UInt32, Int32
from src.types.Str import CStr, FixedLenStr, chk_len

if TYPE_CHECKING:
    from src.generators.IncrementalGenerator import IncrementalGenerator


class FileHeader(BaseSection):
    file_version_str: str = Retriever(FixedLenStr(4), default = "1.47", validators = [partial(chk_len, 4, "file_version_str")])
    header_len: int = Retriever(UInt32, default = 0)
    savable: int = Retriever(Int32, default = 6)
    timestamp_of_last_save: int = Retriever(UInt32, default = 1610675127)
    scenario_instructions: str = Retriever(CStr, default = "")
    player_count: int = Retriever(UInt32, default = 2)
    unknowns: list[int] = Retriever(Array(UInt32), default = [2, 3, 4, 5, 6, 7])
    creator_name: str = Retriever(CStr, default = "KSneijders/AoE2ScenarioParser")
    trigger_count: int = Retriever(UInt32, default = 0)

    def __init__(self, igen: IncrementalGenerator):
        ver_str = igen.get_bytes(4, update_progress = False).decode("ASCII")
        major, minor = map(int, ver_str.split("."))
        self.file_version: tuple[int, int] = major, minor

        super().__init__(igen, self.file_version)
