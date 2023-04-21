from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import str16


class Cinematics(BaseStruct):
    pregame: str = Retriever(str16, default = "")
    victory: str = Retriever(str16, default = "")
    loss: str = Retriever(str16, default = "")

    def __init__(self, struct_ver: Version = Version((1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, parent, initialise_defaults = initialise_defaults, **retriever_inits)
