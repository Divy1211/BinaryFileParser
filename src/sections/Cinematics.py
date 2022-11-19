from __future__ import annotations


from src.retrievers.Retriever import Retriever
from src.types.BaseStruct import BaseStruct
from src.types.Str import str16


class Cinematics(BaseStruct):
    pregame: str = Retriever(str16, default = "")
    victory: str = Retriever(str16, default = "")
    loss: str = Retriever(str16, default = "")

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True):
        super().__init__(struct_version, parent, initialise_defaults)
