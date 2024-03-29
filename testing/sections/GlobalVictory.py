from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import uint32


class GlobalVictory(BaseStruct):
    separator: int = Retriever(uint32, default = 4294967197)
    conquest: int = Retriever(uint32, default = 1)
    """If conquest is checked in 'Custom' GlobalVictory menu"""
    capture_num_monuments: int = Retriever(uint32, default = 0)
    """Ruins in aoe1"""
    collect_num_relics: int = Retriever(uint32, default = 0)
    """Artifacts in aoe1. The value for number of relics that is set when 'relics' is checked in 'Custom' GlobalVictory menu"""
    discovery: int = Retriever(uint32, default = 0)
    """What does this do?"""
    explore_map_percent: int = Retriever(uint32, default = 0)
    """The percentage of the map that needs to be explored for winning in 'Custom' GlobalVictory menu"""
    collect_gold: int = Retriever(uint32, default = 0)
    meet_all_conditions: int = Retriever(uint32, default = 0)
    """When set to 1, all custom conditions must be satisfied to win"""
    mode: int = Retriever(uint32, default = 0)
    min_score: int = Retriever(uint32, default = 900)
    time_limit: int = Retriever(uint32, default = 9000)
    """in 10ths of a year"""

    def __init__(self, struct_ver: Version = Version((1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, parent, initialise_defaults = initialise_defaults, **retriever_inits)
