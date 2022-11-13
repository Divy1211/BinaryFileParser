import zlib

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.retrievers.Retriever import Retriever
from src.sections.BackgroundImage import BackgroundImage
from src.sections.Cinematics import Cinematics
from src.sections.DataHeader import DataHeader
from src.sections.Diplomacy import Diplomacy
from src.sections.FileHeader import FileHeader
from src.sections.Messages import Messages
from src.sections.PlayerData2 import PlayerData2
from src.sections.GlobalVictory import GlobalVictory
from src.types.BaseStruct import BaseStruct


class AoE2Scenario(BaseStruct):
    file_header: FileHeader = Retriever(FileHeader, default = FileHeader())
    data_header: DataHeader = Retriever(DataHeader, default = DataHeader(), remaining_compressed = True)
    messages: Messages = Retriever(Messages, default = Messages())
    cinematics: Cinematics = Retriever(Cinematics, default = Cinematics())
    background_image: BackgroundImage = Retriever(BackgroundImage, default = BackgroundImage())
    player_data_2: PlayerData2 = Retriever(PlayerData2, default = PlayerData2())
    global_victory: GlobalVictory = Retriever(GlobalVictory, default = GlobalVictory())
    diplomacy: Diplomacy = Retriever(Diplomacy, default = Diplomacy())

    @classmethod
    def decompress(cls, bytes_: bytes) -> bytes:
        return zlib.decompress(bytes_, -zlib.MAX_WBITS)

    @classmethod
    def compress(cls, bytes_: bytes) -> bytes:
        deflate_obj = zlib.compressobj(9, zlib.DEFLATED, -zlib.MAX_WBITS)
        compressed = deflate_obj.compress(bytes_) + deflate_obj.flush()
        return compressed

    @classmethod
    def get_version(cls, igen: IncrementalGenerator) -> tuple[int, ...]:
        ver_str = igen.get_bytes(4, update_progress = False).decode("ASCII")
        return tuple(map(int, ver_str.split(".")))

    def __init__(self, version: tuple[int, ...]):
        super().__init__(version)
