from src.retrievers.Retriever import Retriever
from src.types.BaseStruct import BaseStruct
from src.types.Int import UInt32


class GlobalVictory(BaseStruct):
    separator: int = Retriever(UInt32, default = 4294967197)
    conquest: int = Retriever(UInt32, default = 1)
    """If conquest is checked in 'Custom' GlobalVictory menu"""
    ruins: int = Retriever(UInt32, default = 0)
    num_relics: int = Retriever(UInt32, default = 0)
    """The value for number of relics that is set when 'relics' is checked in 'Custom' GlobalVictory menu"""
    discovery: int = Retriever(UInt32, default = 0)
    exploration_percentage: int = Retriever(UInt32, default = 0)
    """The percentage of the map that needs to be explored for winning in 'Custom' GlobalVictory menu"""
    gold: int = Retriever(UInt32, default = 0)
    all_custom_conditions: int = Retriever(UInt32, default = 0)
    """When set to 1, all custom conditions must be satisfied to win"""
    mode: int = Retriever(UInt32, default = 0)
    score: int = Retriever(UInt32, default = 900)
    time_limit: int = Retriever(UInt32, default = 9000)
    """in 10ths of a year"""

    def __init__(self, version: tuple[int, ...] = (1, 47)):
        super().__init__(version)
