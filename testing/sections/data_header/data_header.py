from __future__ import annotations

from binary_file_parser import BaseStruct, ByteStream, Retriever, Version
from binary_file_parser.types import bool32, bool8, Bytes, FixedLenStr, float32, int16, int32, str16, uint16

from testing.sections.data_header.player_base_options import PlayerBaseOptions


class DataHeader(BaseStruct):
    @staticmethod
    def set_mission_items_repeat(_, instance: DataHeader):
        DataHeader.mission_items.set_repeat(instance, instance.num_mission_items)

    @staticmethod
    def unpad_names(_, instance: DataHeader):
        instance.tribe_names = [name.replace("\x00", "") for name in instance.tribe_names]

    @staticmethod
    def pad_names(_, instance: DataHeader):
        instance.tribe_names = [f"{name:\x00<256}" for name in instance.tribe_names]

    # @formatter:off
    version: float =                               Retriever(float32,                                       default = 1.4700000286102295)
    tribe_names: list[str] =                       Retriever(FixedLenStr[256],                              default = "\x00"*256,                repeat = 16, on_read = [unpad_names], on_write = [pad_names])
    player_name_str_ids: list[int] =               Retriever(int32,             min_ver = Version((1, 17)), default = -2,                        repeat = 16)
    player_base_options: list[PlayerBaseOptions] = Retriever(PlayerBaseOptions,                             default_factory = PlayerBaseOptions, repeat = 16)
    lock_civilizations: list[bool] =               Retriever(bool32,            min_ver = Version((1, 28)), default = False,                     repeat = 16)
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
        ver_str = f"{float32._from_bytes(stream.peek(4)):.2f}"
        return Version(map(int, ver_str.split(".")))

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
