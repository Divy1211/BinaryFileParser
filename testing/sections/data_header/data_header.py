from __future__ import annotations

from binary_file_parser import BaseStruct, ByteStream, Retriever, Version
from binary_file_parser.types import bool32, bool8, Bytes, FixedLenStr, float32, int16, int32, str16

from testing.sections.data_header.player_base_options import PlayerBaseOptions


class DataHeader(BaseStruct):
    # @formatter:off
    version: float =                               Retriever(float32,                                                                      default = 1.4700000286102295)
    # todo: pad/truncate length to 256
    tribe_names: list[str] =                       Retriever(FixedLenStr[256],                                                             default = "0"*256,                                     repeat = 16)
    player_name_str_ids: list[int] =               Retriever(int32,                min_ver = Version((1, 16)),                             default = -2,                                          repeat = 16)
    player_base_options: list[PlayerBaseOptions] = Retriever(PlayerBaseOptions,                                                            default_factory = lambda sv: PlayerBaseOptions(sv), repeat = 16)

    # todo: find out when these were removed from the header, 1.35 is temporary because we know for sure it doesn't exist in DE
    victory_conquest: bool =                       Retriever(bool8,                min_ver = Version((1,  7)), max_ver = Version((1, 35)), default = True)
    timeline_count: bool =                         Retriever(int16,                                            max_ver = Version((1, 35)), default = 0)
    timeline_available: bool =                     Retriever(int16,                                            max_ver = Version((1, 35)), default = 0)
    old_time: bool =                               Retriever(float32,                                          max_ver = Version((1, 35)), default = 0)

    lock_civilizations: list[bool] =               Retriever(bool32,               min_ver = Version((1, 28)),                             default = False,                                       repeat = 16)
    unknown: bytes =                               Retriever(Bytes[9],                                         max_ver = Version((1, 45)), default = b"\x00"+b"\x00"*8)
    unknown_1_46: bytes =                          Retriever(Bytes[9],             min_ver = Version((1, 46)),                             default = b"\x01"+b"\x00"*8)
    file_name: str =                               Retriever(str16,                                                                        default = "MadeWithAoE2SP.aoe2scenario")
    # @formatter:on

    @classmethod
    def _get_version(cls, stream: ByteStream, struct_ver: Version = Version((0,))) -> Version:
        # this should be identical to file version, but just in case its possible for it to be different... yES
        ver_str = f"{float32._from_bytes(stream.peek(4)):.3}"
        return Version(map(int, ver_str.split(".")))

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
