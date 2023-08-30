import zlib

from binary_file_parser import BaseStruct, Retriever, Version
from binary_file_parser.types import Bytes, ByteStream
from testing.sections.BackgroundImage import BackgroundImage
from testing.sections.Cinematics import Cinematics
from testing.sections.DataHeader import DataHeader
from testing.sections.Diplomacy import Diplomacy
from testing.sections.FileData import FileData
from testing.sections.FileHeader import FileHeader
from testing.sections.GlobalVictory import GlobalVictory
from testing.sections.MapData import MapData
from testing.sections.Messages import Messages
from testing.sections.Options import Options
from testing.sections.PlayerData2 import PlayerData2
from testing.sections.TriggerData import TriggerData
from testing.sections.UnitData import UnitData


class ScenarioSections(BaseStruct):
    file_header: FileHeader = Retriever(FileHeader, default_factory = lambda sv, p: FileHeader(sv, p))
    data_header: DataHeader = Retriever(DataHeader, default_factory = lambda sv, p: DataHeader(sv, p), remaining_compressed = True)
    messages: Messages = Retriever(Messages, default_factory = lambda sv, p: Messages(sv, p))
    cinematics: Cinematics = Retriever(Cinematics, default_factory = lambda sv, p: Cinematics(sv, p))
    background_image: BackgroundImage = Retriever(BackgroundImage, default_factory = lambda sv, p: BackgroundImage(sv, p))
    player_data2: PlayerData2 = Retriever(PlayerData2, default_factory = lambda sv, p: PlayerData2(sv, p))
    global_victory  : GlobalVictory = Retriever(GlobalVictory, default_factory = lambda sv, p: GlobalVictory(sv, p))
    diplomacy: Diplomacy = Retriever(Diplomacy, default_factory = lambda sv, p: Diplomacy(sv, p))
    options: Options = Retriever(Options, default_factory = lambda sv, p: Options(sv, p))
    map_data: MapData = Retriever(MapData, default_factory = lambda sv, p: MapData(sv, p))
    unit_data: UnitData = Retriever(UnitData, default_factory = lambda sv, p: UnitData(sv, p))
    trigger_data: TriggerData = Retriever(TriggerData, default_factory = lambda sv, p: TriggerData(sv, p))
    file_data: FileData = Retriever(FileData, default_factory = lambda sv, p: FileData(sv, p), min_ver = Version((1, 40)))
    unknown1: bytes = Retriever(Bytes[8], default = b"\x00"*8, max_ver = Version((1, 37)))

    @classmethod
    def decompress(cls, bytes_: bytes) -> bytes:
        return zlib.decompress(bytes_, -zlib.MAX_WBITS)

    @classmethod
    def compress(cls, bytes_: bytes) -> bytes:
        deflate_obj = zlib.compressobj(9, zlib.DEFLATED, -zlib.MAX_WBITS)
        compressed = deflate_obj.compress(bytes_) + deflate_obj.flush()
        return compressed

    @classmethod
    def get_version(
        cls,
        stream: ByteStream,
        struct_ver: Version = Version((0,)),
        parent: BaseStruct = None,
    ) -> Version:
        ver_str = stream.peek(4).decode("ASCII")
        return Version(map(int, ver_str.split(".")))

    def __init__(self, struct_ver: Version = Version((1, 47)), parent: BaseStruct = None, initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, parent, initialise_defaults = initialise_defaults, **retriever_inits)
