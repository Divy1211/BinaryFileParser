from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import str16, uint32


class Messages(BaseStruct):
    instructions_str_id: int = Retriever(uint32, default = 4294967294)
    hints_str_id: int = Retriever(uint32, default = 4294967294)
    victory_str_id: int = Retriever(uint32, default = 4294967294)
    loss_str_id: int = Retriever(uint32, default = 4294967294)
    history_str_id: int = Retriever(uint32, default = 4294967294)
    scouts_str_id: int = Retriever(uint32, default = 4294967294)
    instructions: str = Retriever(str16, default = "")
    hints: str = Retriever(str16, default = "")
    victory: str = Retriever(str16, default = "This scenario was created using AoE2ScenarioParser! Hopefully you enjoyed!")
    loss: str = Retriever(str16, default = "This scenario was created using AoE2ScenarioParser! Hopefully you enjoyed!")
    history: str = Retriever(str16, default = "")
    scouts: str = Retriever(str16, default = "")

    def __init__(self, struct_version: Version = Version((1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_version, parent, initialise_defaults, **retriever_inits)
