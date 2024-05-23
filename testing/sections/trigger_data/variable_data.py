from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import Array32, Bytes
from testing.sections.trigger_data.variable import Variable


class VariableData(BaseStruct):
    variables: list[Variable] = Retriever(Array32[Variable], default_factory = Variable)
    unused: bytes = Retriever(Bytes[9], default = b"\x00"*9, min_ver = Version((3, 0, 1, 46)))
    unknown: bytes = Retriever(Bytes[8], default = b"\x00"*8, min_ver = Version((3, 5, 1, 47)))

    def __init__(self, struct_ver: Version = Version((3, 5, 1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
