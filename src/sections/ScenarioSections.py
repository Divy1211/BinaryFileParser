import zlib

from src.types.IncrementalGenerator import IncrementalGenerator
from src.retrievers.Retriever import Retriever
from src.sections.BackgroundImage import BackgroundImage
from src.sections.Cinematics import Cinematics
from src.sections.DataHeader import DataHeader
from src.sections.Diplomacy import Diplomacy
from src.sections.FileData import FileData
from src.sections.FileHeader import FileHeader
from src.sections.Map import Map
from src.sections.Messages import Messages
from src.sections.Options import Options
from src.sections.PlayerData2 import PlayerData2
from src.sections.GlobalVictory import GlobalVictory
from src.sections.TriggerData import TriggerData
from src.sections.UnitData import UnitData
from src.sections.VariableData import VariableData
from src.types.BaseStruct import BaseStruct


class ScenarioSections(BaseStruct):
    file_header: FileHeader = Retriever(FileHeader, default = FileHeader())
    data_header: DataHeader = Retriever(DataHeader, default = DataHeader(), remaining_compressed = True)
    messages: Messages = Retriever(Messages, default = Messages())
    cinematics: Cinematics = Retriever(Cinematics, default = Cinematics())
    background_image: BackgroundImage = Retriever(BackgroundImage, default = BackgroundImage())
    player_data_2: PlayerData2 = Retriever(PlayerData2, default = PlayerData2())
    global_victory: GlobalVictory = Retriever(GlobalVictory, default = GlobalVictory())
    diplomacy: Diplomacy = Retriever(Diplomacy, default = Diplomacy())
    options: Options = Retriever(Options, default = Options())
    map: Map = Retriever(Map, default = Map())
    unit_data: UnitData = Retriever(UnitData, default = UnitData())
    trigger_data: TriggerData = Retriever(TriggerData, default = TriggerData())
    variable_data: VariableData = Retriever(VariableData, default = VariableData())
    file_data: FileData = Retriever(FileData, default = FileData())

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

    def __init__(self, struct_version: tuple[int, ...] = (1, 47)):
        super().__init__(struct_version)
