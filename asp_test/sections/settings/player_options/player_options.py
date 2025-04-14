from __future__ import annotations

from bfp_rs import BaseStruct, Retriever, Version
from bfp_rs.types.le import NtStr, i32, str16, u8

from asp_test.sections.settings.data_header import PlayerBaseOptions, Resources
from asp_test.sections.settings.player_options.legacy_ai_file import LegacyAiFile
from asp_test.sections.scx_versions import DE_LATEST


class PlayerOptions(BaseStruct):
    # @formatter:off
    build_lists: list[str]                       = Retriever(str16,                                       default = "",                        repeat = 16)
    """unused?"""
    city_plans: list[str]                        = Retriever(str16,                                       default = "",                        repeat = 16)
    """unused?"""
    ai_names: list[str]                          = Retriever(str16,             min_ver = Version(1,  8), default = "",                        repeat = 16)
    legacy_ai_files: list[LegacyAiFile]          = Retriever(LegacyAiFile,                                default_factory = LegacyAiFile,      repeat = 16)
    ai_types: list[int]                          = Retriever(u8,                min_ver = Version(1, 20), default = 1,                         repeat = 16)
    separator1: int                              = Retriever(i32,               min_ver = Version(1,  3), default = -99)
    tribe_names: list[str]                       = Retriever(NtStr[256],        max_ver = Version(1, 13), default = "",                        repeat = 16)
    starting_resources: list[Resources]          = Retriever(Resources,         min_ver = Version(1, 14), default_factory = Resources,         repeat = 16)
    player_base_options: list[PlayerBaseOptions] = Retriever(PlayerBaseOptions, max_ver = Version(1, 13), default_factory = PlayerBaseOptions, repeat = 16)
    separator2: int                              = Retriever(i32,               min_ver = Version(1,  3), default = -99)
    # @formatter:on

    def __new__(cls, ver: Version = DE_LATEST, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, init_defaults, **retriever_inits)
