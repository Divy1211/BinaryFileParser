from __future__ import annotations


from src.retrievers.Retriever import Retriever
from src.types.BaseStruct import BaseStruct
from src.types.Str import Str16


class Cinematics(BaseStruct):
    pregame: str = Retriever(Str16, default = "")
    victory: str = Retriever(Str16, default = "")
    loss: str = Retriever(Str16, default = "")

    def __init__(self, struct_version: tuple[int, ...] = (1, 47)):
        super().__init__(struct_version)
