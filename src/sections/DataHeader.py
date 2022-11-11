from __future__ import annotations

import operator
from functools import partial
from typing import TYPE_CHECKING

from src.retrievers.Retriever import Retriever
from src.sections.BaseSection import BaseSection
from src.types.Float import Float32
from src.types.Int import UInt32, Int32
from src.types.Str import CStr, FixedLenStr, each_len

if TYPE_CHECKING:
    from src.generators.IncrementalGenerator import IncrementalGenerator


class DataHeader(BaseSection):
    next_unit_id: int = Retriever(UInt32, default = 0)
    version: float = Retriever(Float32, default = 1.42)
    tribe_names: list[str] = Retriever(FixedLenStr(256), default = "0"*256, repeat = 16, validators = [partial(each_len, operator.eq, 256)])

    def __init__(self, igen: IncrementalGenerator, file_version: tuple[int]):
        super().__init__(igen, file_version)
