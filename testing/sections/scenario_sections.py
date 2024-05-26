from __future__ import annotations

import zlib
from os import path

from binary_file_parser import BaseStruct, ByteStream, Retriever, Version
from binary_file_parser.types import Bytes, FixedLenStr, str16, uint32
from testing.sections.bitmap import BackgroundImage
from testing.sections.cinematics import Cinematics
from testing.sections.data_header import DataHeader
from testing.sections.diplomacy import Diplomacy
from testing.sections.file_data import FileData
from testing.sections.file_header import FileHeader
from testing.sections.global_victory import GlobalVictory
from testing.sections.map_data import MapData
from testing.sections.messages import Messages
from testing.sections.options import Options
from testing.sections.player_options import PlayerOptions
from testing.sections.trigger_data import TriggerData
from testing.sections.unit_data import UnitData


class ScenarioSections(BaseStruct):
    @staticmethod
    def sync_script_file_path(_, instance: ScenarioSections):
        instance.file_data.script_file_path = instance.options.script_name+".xs" if instance.options.script_name else ""

    @staticmethod
    def sync_num_triggers(_, instance: ScenarioSections):
        instance.file_header.num_triggers = instance.options.num_triggers = len(instance.trigger_data.triggers)

    @staticmethod
    def sync_resources(_, instance: ScenarioSections):
        for i in range(8):
            instance.unit_data.world_player_data[i].food = instance.player_options.starting_resources[i].food
            instance.unit_data.world_player_data[i].wood = instance.player_options.starting_resources[i].wood
            instance.unit_data.world_player_data[i].stone = instance.player_options.starting_resources[i].stone
            instance.unit_data.world_player_data[i].gold = instance.player_options.starting_resources[i].gold
            instance.unit_data.world_player_data[i].ore_x = instance.player_options.starting_resources[i].ore_x
            instance.unit_data.world_player_data[i].trade_goods = instance.player_options.starting_resources[i].trade_goods

    # @formatter:off
    version: str =                      Retriever(FixedLenStr[4],                              default = "1.47")
    file_header: FileHeader =           Retriever(FileHeader,                                  default_factory = FileHeader, on_write = [sync_num_triggers])
    next_unit_ref: int =                Retriever(uint32,                                      default = 0,                  remaining_compressed = True)
    data_header: DataHeader =           Retriever(DataHeader,                                  default_factory = DataHeader)
    messages: Messages =                Retriever(Messages,                                    default_factory = Messages)
    cinematics: Cinematics =            Retriever(Cinematics,                                  default_factory = Cinematics)
    background_image_filename: str =    Retriever(str16,           min_ver = Version((1,  9)), default = "")
    background_image: BackgroundImage = Retriever(BackgroundImage, min_ver = Version((1, 10)), default_factory = BackgroundImage)
    player_options: PlayerOptions =     Retriever(PlayerOptions,                               default_factory = PlayerOptions)
    global_victory: GlobalVictory =     Retriever(GlobalVictory,                               default_factory = GlobalVictory)
    diplomacy: Diplomacy =              Retriever(Diplomacy,                                   default_factory = Diplomacy)
    options: Options =                  Retriever(Options,                                     default_factory = Options)
    map_data: MapData =                 Retriever(MapData,                                     default_factory = MapData)
    unit_data: UnitData =               Retriever(UnitData,                                    default_factory = UnitData,   on_write = [sync_resources])
    trigger_data: TriggerData =         Retriever(TriggerData,     min_ver = Version((1, 14)), default_factory = TriggerData)
    file_data: FileData =               Retriever(FileData,        min_ver = Version((1, 17)), default_factory = FileData,   on_write = [sync_script_file_path])
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
        struct_ver: Version = Version((0,)),
    ) -> Version:
        ver_str = stream.peek(4).decode("ASCII")
        return Version(map(int, ver_str.split(".")))

    @classmethod
    def from_file(cls, file_name: str, *, file_version: Version = Version((0,)), strict = True) -> ScenarioSections:
        return cls._from_file(file_name, file_version = file_version, strict = strict)

    def to_file(self, file_name: str, overwrite_original_file_name = True):
        if overwrite_original_file_name:
            self.data_header.file_name = path.basename(file_name)
        super()._to_file(file_name)

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)

#         self.player_man = PlayerManager(self)
#
#
# class PlayerManager:
#     _num_players =         RetrieverRef(ScenarioSections.file_header, FileHeader.num_players)
#     _tribe_names =         RetrieverRef(ScenarioSections.data_header, DataHeader.tribe_names)
#     _player_name_str_ids = RetrieverRef(ScenarioSections.data_header, DataHeader.player_name_str_ids)
#     _metadata =            RetrieverRef(ScenarioSections.data_header, DataHeader.player_base_options)
#     _lock_civilizations =  RetrieverRef(ScenarioSections.data_header, DataHeader.lock_civilizations)
#     _resources =           RetrieverRef(ScenarioSections.player_options, PlayerOptions.starting_resources)
#     _player_stances =      RetrieverRef(ScenarioSections.diplomacy, Diplomacy.player_stances)
#
#     def __init__(self, struct: ScenarioSections):
#         self._struct = struct
