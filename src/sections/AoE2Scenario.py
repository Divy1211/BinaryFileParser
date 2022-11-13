import zlib

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.retrievers.Retriever import Retriever
from src.sections.BackgroundImage import BackgroundImage
from src.sections.Cinematics import Cinematics
from src.sections.DataHeader import DataHeader
from src.sections.FileHeader import FileHeader
from src.sections.Messages import Messages
from src.types.BaseStruct import BaseStruct


class AoE2Scenario(BaseStruct):
    file_header: FileHeader = Retriever(FileHeader, default = FileHeader())
    data_header: DataHeader = Retriever(DataHeader, default = DataHeader(), remaining_compressed = True)
    messages: Messages = Retriever(Messages, default = Messages())
    cinematics: Cinematics = Retriever(Cinematics, default = Cinematics())
    background_image: BackgroundImage = Retriever(BackgroundImage, default = BackgroundImage())

    @classmethod
    def decompress(cls, bytes_: bytes) -> bytes:
        return zlib.decompress(bytes_, -zlib.MAX_WBITS)

    @classmethod
    def compress(cls, bytes_: bytes) -> bytes:
        deflate_obj = zlib.compressobj(9, zlib.DEFLATED, -zlib.MAX_WBITS)
        compressed = deflate_obj.compress(bytes_) + deflate_obj.flush()
        return compressed

    @classmethod
    def get_file_version(cls, igen: IncrementalGenerator) -> tuple[int, ...]:
        ver_str = igen.get_bytes(4, update_progress = False).decode("ASCII")
        return tuple(map(int, ver_str.split(".")))

    def __init__(self, file_version: tuple[int, ...]):
        super().__init__(file_version)
