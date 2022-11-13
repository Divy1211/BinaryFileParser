from src.retrievers.Retriever import Retriever
from src.types.Array import FixedLenArray
from src.types.BaseStruct import BaseStruct
from src.types.Bool import Bool32, Bool8
from src.types.Bytes import Bytes
from src.types.Int import UInt32, Int8


class Diplomacy(BaseStruct):
    player_stances: list[list[int]] = Retriever(FixedLenArray(16, UInt32), default = [3]*16, repeat = 16)
    unused: bytes = Retriever(Bytes(11520), default = b"\x00"*11520)
    separator: int = Retriever(UInt32, default = 4294967197)
    allied_victory: list[bool] = Retriever(Bool32, default = False, repeat = 16)
    lock_teams_in_game: bool = Retriever(Bool8, default = False)
    lock_teams_in_lobby: bool = Retriever(Bool8, default = False)
    random_start_points: bool = Retriever(Bool8, default = False)
    max_num_teams: int = Retriever(Int8, default = 4)

    def __init__(self, version: tuple[int, ...] = (1, 47)):
        super().__init__(version)
