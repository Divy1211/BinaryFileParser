from __future__ import annotations

from binary_file_parser import BaseStruct, ByteStream, Retriever, Version
from binary_file_parser.types import (
    bool32, bool8, Bytes, FixedLenNTStr, float32, int32, str16,
    uint16, uint32,
)
from testing.sections.data_header.player_base_options import PlayerBaseOptions
from testing.sections.scx_versions import DE_LATEST


class DataHeader(BaseStruct):
    @staticmethod
    def set_mission_items_repeat(_, instance: DataHeader):
        DataHeader.mission_items.set_repeat(instance, instance.num_mission_items)

    # @formatter:off
    next_unit_ref: int =                           Retriever(uint32,                                        default = 0)
    version: float =                               Retriever(float32,                                       default = 1.53)
    num_max_players: int =                         Retriever(int32,             min_ver = Version((1, 53)), default = 0)
    gaia_player_idx: int =                         Retriever(int32,             min_ver = Version((1, 53)), default = 0)
    tribe_names: list[str] =                       Retriever(FixedLenNTStr[256],                            default = "",                        repeat = 16)
    player_name_str_ids: list[int] =               Retriever(int32,             min_ver = Version((1, 17)), default = -2,                        repeat = 16)
    player_base_options: list[PlayerBaseOptions] = Retriever(PlayerBaseOptions,                             default_factory = PlayerBaseOptions, repeat = 16)
    lock_civilizations: list[bool] =               Retriever(bool32,            min_ver = Version((1, 28)), default = False,                     repeat = 16)
    lock_ai_personality: list[bool] =              Retriever(bool32,            min_ver = Version((1, 53)), default = False,                     repeat = 16)
    victory_conquest: bool =                       Retriever(bool8,             min_ver = Version((1,  7)), default = True)
    num_mission_items: int =                       Retriever(uint16,                                        default = 0,                                      on_set = [set_mission_items_repeat])
    """what does this do?"""
    mission_available: int =                       Retriever(uint16,                                        default = 0)
    """what does this do?"""
    mission_timeline: float =                      Retriever(float32,                                       default = 0)
    """what does this do?"""
    mission_items: list[bytes] =                   Retriever(Bytes[30],                                     default = b"\x00"*30,                repeat = 0)
    """what does this do?"""
    file_name: str =                               Retriever(str16,                                         default = "MadeWithAoE2SP.aoe2scenario")
    # @formatter:on

    @classmethod
    def _get_version(cls, stream: ByteStream, struct_ver: Version = Version((0,))) -> Version:
        # this should be identical to file version, but just in case its possible for it to be different... yES
        ver_str = f"{float32._from_bytes(stream.peek(8)[4:]):.2f}"
        return Version(map(int, ver_str.split(".")))

    def __init__(self, struct_ver: Version = DE_LATEST, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
