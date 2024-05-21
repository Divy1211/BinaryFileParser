from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import bool32, bool8, Bytes, FixedLenStr, float32, int16, str16, uint32

from testing.sections.data_header.player_base_properties import PlayerBaseProperties


class DataHeader(BaseStruct):
    # @formatter:off
    # todo: fix max_ver version comparison behaviour, use _get_version here
    version: float =                                     Retriever(float32,                                          default = 1.4700000286102295)
    # todo: pad/truncate length to 256
    tribe_names: list[str] =                             Retriever(FixedLenStr[256],     min_ver = Version((1, 13)), default = "0"*256, repeat = 16)
    player_name_str_ids: list[int] =                     Retriever(uint32,               min_ver = Version((1, 16)), default = 4294967294, repeat = 16)
    player_base_properties: list[PlayerBaseProperties] = Retriever(PlayerBaseProperties,                             default_factory = lambda sv: PlayerBaseProperties(sv), repeat = 16)

    # todo: find out when these were removed from the header, 1.35 is temporary because we know for sure it doesn't exist in DE
    victory_conquest: bool =                             Retriever(bool8,                max_ver = Version((1, 35)), default = True)
    timeline_count: bool =                               Retriever(int16,                max_ver = Version((1, 35)), default = 0)
    timeline_available: bool =                           Retriever(int16,                max_ver = Version((1, 35)), default = 0)
    old_time: bool =                                     Retriever(float32,              max_ver = Version((1, 35)), default = 0)

    lock_civilizations: list[bool] =                     Retriever(bool32,               min_ver = Version((1, 28)), default = False, repeat = 16)
    unknown: bytes =                                     Retriever(Bytes[9],             max_ver = Version((1, 45)), default = b"\x00"+b"\x00"*8)
    unknown_1_46: bytes =                                Retriever(Bytes[9],             min_ver = Version((1, 46)), default = b"\x01"+b"\x00"*8)
    file_name: str =                                     Retriever(str16,                                            default = "MadeWithAoE2SP.aoe2scenario")
    # @formatter:on

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
