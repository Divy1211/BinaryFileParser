from __future__ import annotations


from src.retrievers.Retriever import Retriever
from src.types.BaseStruct import BaseStruct
from src.types.Int import UInt32
from src.types.Str import Str16


class Messages(BaseStruct):
    instructions_str_id: int = Retriever(UInt32, default = 4294967294)
    hints_str_id: int = Retriever(UInt32, default = 4294967294)
    victory_str_id: int = Retriever(UInt32, default = 4294967294)
    loss_str_id: int = Retriever(UInt32, default = 4294967294)
    history_str_id: int = Retriever(UInt32, default = 4294967294)
    scouts_str_id: int = Retriever(UInt32, default = 4294967294)
    instructions: str = Retriever(Str16, default = "")
    hints: str = Retriever(Str16, default = "")
    victory: str = Retriever(Str16, default = "This scenario was created using AoE2ScenarioParser! Hopefully you enjoyed!")
    loss: str = Retriever(Str16, default = "This scenario was created using AoE2ScenarioParser! Hopefully you enjoyed!")
    history: str = Retriever(Str16, default = "")
    scouts: str = Retriever(Str16, default = "")

    def __init__(self, version: tuple[int, ...] = (1, 47)):
        super().__init__(version)
