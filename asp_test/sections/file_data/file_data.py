from __future__ import annotations

from bfp_rs import BaseStruct, Retriever, Version
from bfp_rs.types.le import Array32, bool32, str16, str32
from bfp_rs.combinators import set_repeat, if_not, if_len, set_

from asp_test.sections.file_data.ai_error import AiError
from asp_test.sections.file_data.ai_file import AiFile

from asp_test.sections.scx_versions import DE_LATEST


class FileData(BaseStruct):
    # @formatter:off
    has_script_files: bool   = Retriever(bool32,           min_ver = Version(1, 40), max_ver = Version(1, 45), default = False)
    script_file_path: str    = Retriever(str16,            min_ver = Version(1, 40),                           default = "")
    script: str              = Retriever(str32,            min_ver = Version(1, 40),                           default = "")
    has_ai_files: bool       = Retriever(bool32,                                                               default = False, on_read = lambda: [if_not(FileData.has_ai_files).then(set_repeat(FileData.ai_files).to(0))], on_write = lambda: [if_len(FileData.ai_files).gt(0).then(set_(FileData.has_ai_files).to(True))])
    has_ai_errors: bool      = Retriever(bool32,                                                               default = False, on_read = lambda: [if_not(FileData.has_ai_errors).then(set_repeat(FileData.ai_errors).to(0))], on_write = lambda: [if_len(FileData.ai_errors).gt(0).then(set_(FileData.has_ai_errors).to(True))])
    ai_errors: list[AiError] = Retriever(Array32[AiError],                                                     default_factory = lambda _ver: [])
    ai_files: list[AiFile]   = Retriever(Array32[AiFile],                                                      default_factory = lambda _ver: [])
    # @formatter:on

    def __new__(cls, ver: Version = DE_LATEST, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, init_defaults, **retriever_inits)
