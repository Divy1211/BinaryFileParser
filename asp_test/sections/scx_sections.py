import zlib

from bfp_rs import BaseStruct, Retriever, ByteStream, Version
from bfp_rs.combinators import set_, if_ver

from asp_test.sections.scx_versions import DE_LATEST
from asp_test.sections.file_header import FileHeader
from asp_test.sections.settings import Settings, PlayerOptions, Resources
from asp_test.sections.map_data import MapData
from asp_test.sections.unit_data import UnitData, WorldPlayerData
from asp_test.sections.trigger_data import TriggerData
from asp_test.sections.file_data import FileData


class ScenarioSections(BaseStruct):
    # @formatter:off
    file_header: FileHeader   = Retriever(FileHeader,                                default_factory = lambda _ver: FileHeader())
    settings: Settings        = Retriever(Settings,                                  default_factory = Settings,   remaining_compressed = True)
    map_data: MapData         = Retriever(MapData,                                   default_factory = lambda _ver: MapData())
    unit_data: UnitData       = Retriever(UnitData,                                  default_factory = UnitData,   on_write = lambda: [*[set_(ScenarioSections.unit_data, UnitData.world_player_data, i, getattr(WorldPlayerData, prop)).from_(ScenarioSections.settings, Settings.player_options, PlayerOptions.starting_resources, i, getattr(Resources, prop)) for i in range(8) for prop in ["food", "wood", "stone", "gold"]], *[if_ver(min = Version(1, 18)).then(set_(ScenarioSections.unit_data, UnitData.world_player_data, i, getattr(WorldPlayerData, prop)).from_(ScenarioSections.settings, Settings.player_options, PlayerOptions.starting_resources, i, getattr(Resources, prop))) for i in range(8) for prop in ["ore_x", "trade_goods"]]])
    trigger_data: TriggerData = Retriever(TriggerData,     min_ver = Version(1, 14), default_factory = TriggerData)
    file_data: FileData       = Retriever(FileData,        min_ver = Version(1, 17), default_factory = FileData)
    # @formatter:on

    @classmethod
    def _decompress(cls, bytes_: bytes) -> bytes:
        return zlib.decompress(bytes_, -zlib.MAX_WBITS)

    @classmethod
    def _compress(cls, bytes_: bytes) -> bytes:
        deflate_obj = zlib.compressobj(9, zlib.DEFLATED, -zlib.MAX_WBITS)
        compressed = deflate_obj.compress(bytes_) + deflate_obj.flush()
        return compressed

    @classmethod
    def _get_version(
        cls,
        stream: ByteStream,
        _ver: Version = Version(0),
    ) -> Version:
        ver_str = stream.peek(4).decode("ASCII")
        return Version(*map(int, ver_str.split(".")))

    def __new__(cls, ver: Version = DE_LATEST, init_defaults = True, **retriever_inits):
        return super().__new__(cls, ver, init_defaults, **retriever_inits)
