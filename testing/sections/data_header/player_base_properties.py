from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, RetrieverCombiner, Version
from binary_file_parser.types import bool32, uint32

class PlayerBaseProperties(BaseStruct):
    active: bool = Retriever(bool32, default = False)
    human: bool =  Retriever(bool32, default = False)

    # todo: add correct defaults for different versions, or there will be crashes in DE
    _civilization_1_36: int = Retriever(uint32, default = 36, min_ver = Version((1, 36)), max_ver = Version((1, 40)))
    _civilization_1_41: int = Retriever(uint32, default = 38, min_ver = Version((1, 41)), max_ver = Version((1, 42)))
    _civilization_1_43: int = Retriever(uint32, default = 40, min_ver = Version((1, 43)), max_ver = Version((1, 45)))
    _civilization_1_46: int = Retriever(uint32, default = 43, min_ver = Version((1, 46)))

    _architecture_1_40: int = Retriever(uint32, default = 36, min_ver = Version((1, 40)), max_ver = Version((1, 40)))
    _architecture_1_41: int = Retriever(uint32, default = 38, min_ver = Version((1, 41)), max_ver = Version((1, 42)))
    _architecture_1_43: int = Retriever(uint32, default = 40, min_ver = Version((1, 43)), max_ver = Version((1, 45)))
    _architecture_1_46: int = Retriever(uint32, default = 43, min_ver = Version((1, 46)))

    posture: int = Retriever(uint32, default = 4)

    civilization = RetrieverCombiner(_civilization_1_36, _civilization_1_41, _civilization_1_43, _civilization_1_46)
    architecture = RetrieverCombiner(_architecture_1_40, _architecture_1_41, _architecture_1_43, _architecture_1_46)

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)