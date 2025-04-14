from __future__ import annotations

from bfp_rs import BaseStruct, Retriever, Version
from bfp_rs.types.le import (
    bool32, bool8, NtStr, f32, i32, str16,
    u16, u32,
)
from asp_test.sections.settings.data_header.player_base_options import PlayerBaseOptions
from asp_test.sections.scx_versions import DE_LATEST


class DataHeader(BaseStruct):
    # @formatter:off
    next_unit_ref: int                           = Retriever(u32,                                          default = 0)
    version: float                               = Retriever(f32,                                          default = 1.54)
    num_max_players: int                         = Retriever(i32,                min_ver = Version(1, 52), default = 0)
    gaia_player_idx: int                         = Retriever(i32,                min_ver = Version(1, 52), default = 0)
    tribe_names: list[str]                       = Retriever(NtStr[256],         min_ver = Version(1, 13), default = "",                        repeat = 16)
    player_name_str_ids: list[int]               = Retriever(i32,                min_ver = Version(1, 16), default = -2,                        repeat = 16)
    player_base_options: list[PlayerBaseOptions] = Retriever(PlayerBaseOptions,  min_ver = Version(1, 14), default_factory = PlayerBaseOptions, repeat = 16)
    lock_civilizations: list[bool]               = Retriever(bool32,             min_ver = Version(1, 28), default = False,                     repeat = 16)
    lock_ai_personality: list[bool]              = Retriever(bool32,             min_ver = Version(1, 53), default = False,                     repeat = 16)
    victory_conquest: bool                       = Retriever(bool8,              min_ver = Version(1,  7), default = True)
    timeline_count: int                          = Retriever(u16,                                          default = 0)
    """unused, must always be 0"""
    timeline_available: int                      = Retriever(u16,                                          default = 0)
    """unused"""
    old_timeline: float                          = Retriever(f32,                                          default = 0)
    """unused"""
    file_name: str                               = Retriever(str16,                                        default = "MadeWithAoE2SP.aoe2scenario")
    # @formatter:on

    def __new__(cls, ver: Version = DE_LATEST, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, init_defaults, **retriever_inits)
