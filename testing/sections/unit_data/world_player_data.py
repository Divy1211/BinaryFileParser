from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import float32


class WorldPlayerData(BaseStruct):
    """All data here is duplicated except pop limit"""

    # @formatter:off
    food: float =             Retriever(float32, min_ver = Version(()), default = 200)
    wood: float =             Retriever(float32, min_ver = Version(()), default = 200)
    gold: float =             Retriever(float32, min_ver = Version(()), default = 200)
    stone: float =            Retriever(float32, min_ver = Version(()), default = 200)
    ore_x: float =            Retriever(float32, min_ver = Version(()), default = 100)
    trade_goods: float =      Retriever(float32, min_ver = Version(()), default = 0.0)
    population_limit: float = Retriever(float32, min_ver = Version(()), default = 200.0)
    # @formatter:on

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)