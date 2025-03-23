from bfp_rs import BaseStruct, Retriever, Version
from bfp_rs.types.le import bool32, bool8, Array, i32, i8, u32

from asp_test.sections.settings.diplomacy.legacy_victory_info import LegacyVictoryInfo
from asp_test.sections.scx_versions import DE_LATEST


class Diplomacy(BaseStruct):
    # @formatter:off
    player_stances: list[list[int]]                    = Retriever(Array[16][u32],                   default_factory = lambda _: [3]*16,                                      repeat = 16)
    legacy_victory_info: list[list[LegacyVictoryInfo]] = Retriever(Array[12][LegacyVictoryInfo],     default_factory = lambda sv: [LegacyVictoryInfo(sv) for _ in range(12)], repeat = 16)
    """used in aoe1"""
    separator: int                                     = Retriever(i32,    min_ver = Version(1,  2), default = -99)
    allied_victories: list[bool]                       = Retriever(bool32,                           default = False,                                                         repeat = 16)
    lock_teams_in_game: bool                           = Retriever(bool8,  min_ver = Version(1, 23), default = False)
    lock_teams_in_lobby: bool                          = Retriever(bool8,  min_ver = Version(1, 24), default = False)
    random_start_points: bool                          = Retriever(bool8,  min_ver = Version(1, 24), default = False)
    max_num_teams: int                                 = Retriever(i8,     min_ver = Version(1, 24), default = 4)
    # @formatter:on

    def __new__(cls, ver: Version = DE_LATEST, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, init_defaults, **retriever_inits)
