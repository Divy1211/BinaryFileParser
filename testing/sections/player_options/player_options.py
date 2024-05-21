from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import int32, str16, uint8
from testing.sections.player_options.ai_file import AiFile
from testing.sections.player_options.resources import Resources


class PlayerOptions(BaseStruct):
    # @formatter:off
    build_lists: list[str] =              Retriever(str16,                                 default = "",                               repeat = 16)
    """unused"""
    city_plans: list[str] =               Retriever(str16,                                 default = "",                               repeat = 16)
    """unused"""
    ai_names: list[str] =                 Retriever(str16,     min_ver = Version((1,  8)), default = "",                               repeat = 16)
    ai_files: list[AiFile] =              Retriever(AiFile,                                default_factory = lambda sv: AiFile(sv),    repeat = 16)
    ai_types: list[int] =                 Retriever(uint8,     min_ver = Version((1, 20)), default = 1,                                repeat = 16)
    separator: int =                      Retriever(int32,     min_ver = Version((1,  2)), default = -99)
    starting_resources: list[Resources] = Retriever(Resources, min_ver = Version((1, 14)), default_factory = lambda sv: Resources(sv), repeat = 16)
    # @formatter:on

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
