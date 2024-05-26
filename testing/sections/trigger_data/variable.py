from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import nt_str32, uint32


class Variable(BaseStruct):
    # @formatter:off
    id: int =   Retriever(uint32, default = 0)
    name: str = Retriever(nt_str32, default = "_Variable")
    # @formatter:on

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
