from __future__ import annotations

from src.retrievers.Retriever import Retriever
from src.types.Array import Array32
from src.types.BaseStruct import BaseStruct
from src.types.Bool import bool32
from src.types.Bytes import Bytes
from src.types.Str import str32, str16, nt_str32


class AiFile2(BaseStruct):
    file_name: str = Retriever(nt_str32, default = "")
    per_content: str = Retriever(nt_str32, default = "")

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None):
        super().__init__(struct_version, parent)

class FileData(BaseStruct):
    @staticmethod
    def set_ai_files_repeat(retriever: Retriever, instance: FileData):
        if not instance.ai_files_present:
            FileData.ai_files.set_repeat(instance, 0)

    @staticmethod
    def update_ai_files_present(retriever: Retriever, instance: FileData):
        instance.ai_files_present = len(instance.ai_files) > 0

    script_file_path: str = Retriever(str16, default = "")
    script_file_content: str = Retriever(str32, default = "")
    ai_files_present: bool = Retriever(bool32, default = False, on_set = [set_ai_files_repeat], on_write = [update_ai_files_present])
    unknown4: bytes = Retriever(Bytes[4], default = b"\x00"*4)
    ai_files: list[AiFile2] = Retriever(Array32[AiFile2], default = [])

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None):
        super().__init__(struct_version, parent)
