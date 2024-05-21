from __future__ import annotations

from binary_file_parser import BaseStruct, ByteStream, Retriever, Version
from binary_file_parser.types import bool32, int32, nt_str32, uint32

from testing.sections.file_header.dlc_options import DLCOptions


class FileHeader(BaseStruct):
    # @formatter:off
    # todo: this should be set from the # of bytes in this header, but it is unused
    size: int =                       Retriever(uint32,                               default = 0)
    version: int =                    Retriever(int32,                                default = 6)
    timestamp_of_last_save: int =     Retriever(uint32,     min_ver = Version((2, )), default = 1610675127)
    scenario_instructions: str =      Retriever(nt_str32,                             default = "")
    individual_victories_used: bool = Retriever(bool32,     max_ver = Version((5, )), default = False)
    num_players: int =                Retriever(uint32,                               default = 2)
    dlc_options: DLCOptions =         Retriever(DLCOptions, min_ver = Version((3, )), default_factory = lambda sv: DLCOptions())
    creator: str =                    Retriever(nt_str32,   min_ver = Version((5, )), default = "AoE2SP")
    num_triggers: int =               Retriever(uint32,     min_ver = Version((5, )), default = 0)
    # @formatter:on

    @classmethod
    def _get_version(cls, stream: ByteStream, struct_ver: Version = Version((0,))) -> Version:
        ver = int32._from_bytes(stream.peek(8)[4:])
        return Version((ver, ))

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
