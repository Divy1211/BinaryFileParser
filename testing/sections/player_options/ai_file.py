from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import str32


class AiFile(BaseStruct):
    # @formatter:off
    build_list: str = Retriever(str32, default = "")
    """unused"""
    city_plans: str = Retriever(str32, default = "")
    """unused"""
    ai_rules: str =   Retriever(str32, default = "", min_ver = Version((1,  8)))
    # @formatter:on

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
