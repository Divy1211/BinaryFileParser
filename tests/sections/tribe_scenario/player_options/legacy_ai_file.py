from __future__ import annotations

from binary_file_parser import BaseStruct, ByteStream, Retriever, Version
from binary_file_parser.types import FixedLenStr, int32
from tests.sections.scx_versions import DE_LATEST


class LegacyAiFile(BaseStruct):
    """This struct is useless"""
    @staticmethod
    def set_len(retriever: Retriever, instance: LegacyAiFile):
        target_ret: Retriever = getattr(LegacyAiFile, retriever.p_name[1:-4])
        target_ret.dtype.length = getattr(instance, retriever.p_name)

    @staticmethod
    def sync_len(retriever: Retriever, instance: LegacyAiFile):
        source_ret: Retriever = getattr(LegacyAiFile, retriever.p_name[1:-4])
        len_ = len(getattr(instance, source_ret.p_name))
        source_ret.dtype.length = len_
        setattr(instance, retriever.p_name, len_)

    # @formatter:off
    _build_list_len: int = Retriever(int32, default = 0,                             on_read = [set_len], on_write = [sync_len])
    _city_plans_len: int = Retriever(int32, default = 0,                             on_read = [set_len], on_write = [sync_len])
    _ai_rules_len: int =   Retriever(int32, default = 0, min_ver = Version((1,  8)), on_read = [set_len], on_write = [sync_len])

    build_list: str =      Retriever(FixedLenStr[0], default = "")
    """unused?"""
    city_plans: str =      Retriever(FixedLenStr[0], default = "")
    """unused?"""
    ai_rules: str =        Retriever(FixedLenStr[0], default = "", min_ver = Version((1,  8)))
    """From the .per file of an AI"""
    # @formatter:on

    def __init__(self, struct_ver: Version = DE_LATEST, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
