from binary_file_parser import BaseStruct, ByteStream, Retriever, RetrieverCombiner, Version
from binary_file_parser.types import Array32, uint32


class DLCOptions(BaseStruct):
    # @formatter:off
    version: int = Retriever(uint32, default = 1000)
    """
    - 0    - AoC   -- this used to be the game_dataset before this struct was versioned
    - 1    - HD+
    - 1000 - HD/DE -- actual value of version for DE/HD
    """
    _game_dataset: int =       Retriever(uint32, default = 1, min_ver = Version((1000, )))
    # todo: update this list with proper default versioning
    required_dlcs: list[int] = Retriever(Array32[uint32], default_factory = lambda _, __: [2, 3, 4, 5, 6, 7], min_ver = Version((1000, )))
    """
    - 2  - AoK
    - 3  - AoC
    - 4  - FE
    - 5  - AK
    - 6  - RoR
    - 7  - TLK
    - 8  - LotW
    - 9  - DotD
    - 10 - DI
    - 11 - RoR
    - 12 - TMR
    """

    game_dataset: int = RetrieverCombiner(_game_dataset, version)
    # @formatter:on

    @classmethod
    def _get_version(cls, stream: ByteStream, struct_ver: Version = Version((0,))) -> Version:
        ver = uint32._from_bytes(stream.peek(4))
        return Version((ver, )) + struct_ver

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)