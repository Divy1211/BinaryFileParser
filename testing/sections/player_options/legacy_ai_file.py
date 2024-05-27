from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import str32
from testing.sections.scx_versions import DE_LATEST


class LegacyAiFile(BaseStruct):
    """This struct is useless"""

    # @formatter:off
    # Note: these are potentially stacked32strs, since build_ls and cty_plans are unused, we don't know (or care)
    build_list: str = Retriever(str32, default = "")
    """unused"""
    city_plans: str = Retriever(str32, default = "")
    """unused"""
    ai_rules: str =   Retriever(str32, default = "", min_ver = Version((1,  8)))  # min ver unverified
    """From the .per file of an AI"""
    # @formatter:on

    def __init__(self, struct_ver: Version = DE_LATEST, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
