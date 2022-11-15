from __future__ import annotations

from src.retrievers.Retriever import Retriever
from src.types.Array import Array32
from src.types.BaseStruct import BaseStruct
from src.types.Bool import Bool32
from src.types.Bytes import Bytes
from src.types.Str import Str32, Str16, NullTermStr32


class AiFile2(BaseStruct):
    file_name: str = Retriever(NullTermStr32, default = "")
    per_content: str = Retriever(NullTermStr32, default = "")

    def __init__(self, struct_version: tuple[int, ...] = (3, 2)):
        super().__init__(struct_version)

class FileData(BaseStruct):
    @staticmethod
    def set_ai_files_repeat(retriever: Retriever, instance: FileData):
        if not instance.ai_files_present:
            FileData.ai_files.set_repeat(instance, 0)

    @staticmethod
    def update_ai_files_present(retriever: Retriever, instance: FileData):
        instance.ai_files_present = len(instance.ai_files) > 0

    script_file_path: str = Retriever(Str16, default = "")
    script_file_content: str = Retriever(Str32, default = "")
    ai_files_present: bool = Retriever(Bool32, default = False, on_set = [set_ai_files_repeat], on_write = [update_ai_files_present])
    unknown4: bytes = Retriever(Bytes[4], default = b"\x00"*4)
    ai_files: list[AiFile2] = Retriever(Array32[AiFile2], default = [])

    def __init__(self, struct_version: tuple[int, ...] = (3, 2)):
        super().__init__(struct_version)