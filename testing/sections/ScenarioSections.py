from __future__ import annotations

import zlib

from binary_file_parser import BaseStruct, ByteStream, Retriever, RetrieverRef, Version
from binary_file_parser.types import Bytes
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
    @staticmethod
    def sync_script_file_path(retriever: Retriever, instance: ScenarioSections):
        instance.file_data.script_file_path = instance.map_data.script_name+".xs" if instance.map_data.script_name else ""

    @staticmethod
    def sync_num_triggers(retriever: Retriever, instance: ScenarioSections):
        instance.file_header.num_triggers = instance.options.num_triggers = len(instance.trigger_data.triggers)

    @staticmethod
    def sync_resources(retriever: Retriever, instance: ScenarioSections):
        for i in range(8):
            instance.unit_data.player_data4[i].food = instance.player_data2.resources[i].food
            instance.unit_data.player_data4[i].wood = instance.player_data2.resources[i].wood
            instance.unit_data.player_data4[i].stone = instance.player_data2.resources[i].stone
            instance.unit_data.player_data4[i].gold = instance.player_data2.resources[i].gold
            instance.unit_data.player_data4[i].ore_x = instance.player_data2.resources[i].ore_x
            instance.unit_data.player_data4[i].trade_goods = instance.player_data2.resources[i].trade_goods

    file_header: FileHeader = Retriever(FileHeader, default_factory = lambda sv, p: FileHeader(sv, p), on_write = [sync_num_triggers])
    data_header: DataHeader = Retriever(DataHeader, default_factory = lambda sv, p: DataHeader(sv, p), remaining_compressed = True)
    messages: Messages = Retriever(Messages, default_factory = lambda sv, p: Messages(sv, p))
    cinematics: Cinematics = Retriever(Cinematics, default_factory = lambda sv, p: Cinematics(sv, p))
    background_image: BackgroundImage = Retriever(BackgroundImage, default_factory = lambda sv, p: BackgroundImage(sv, p))
    player_data2: PlayerData2 = Retriever(PlayerData2, default_factory = lambda sv, p: PlayerData2(sv, p))
    global_victory: GlobalVictory = Retriever(GlobalVictory, default_factory = lambda sv, p: GlobalVictory(sv, p))
    diplomacy: Diplomacy = Retriever(Diplomacy, default_factory = lambda sv, p: Diplomacy(sv, p))
    options: Options = Retriever(Options, default_factory = lambda sv, p: Options(sv, p))
    map_data: MapData = Retriever(MapData, default_factory = lambda sv, p: MapData(sv, p))
    unit_data: UnitData = Retriever(UnitData, default_factory = lambda sv, p: UnitData(sv, p), on_write = [sync_resources])
    trigger_data: TriggerData = Retriever(TriggerData, default_factory = lambda sv, p: TriggerData(sv, p))
    file_data: FileData = Retriever(FileData, default_factory = lambda sv, p: FileData(sv, p), min_ver = Version((1, 40)), on_write = [sync_script_file_path])
    unknown1: bytes = Retriever(Bytes[8], default = b"\x00"*8, max_ver = Version((1, 37)))

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
    def from_file(cls, file_name: str, *, file_version: Version = Version((0,)), strict = True) -> BaseStruct:
        return cls._from_file(file_name, file_version = file_version, strict = strict)

    def to_file(self, file_name: str):
        super()._to_file(file_name)

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)

        self.player_man = PlayerManager(self)


class PlayerManager:
    _num_players =        RetrieverRef(ScenarioSections.file_header, FileHeader.num_players) # type: ignore

    _tribe_names =         RetrieverRef(ScenarioSections.data_header, DataHeader.tribe_names) # type: ignore
    _player_name_str_ids = RetrieverRef(ScenarioSections.data_header, DataHeader.player_name_str_ids) # type: ignore
    _metadata            = RetrieverRef(ScenarioSections.data_header, DataHeader.player_data1) # type: ignore
    _lock_civilizations  = RetrieverRef(ScenarioSections.data_header, DataHeader.lock_civilizations) # type: ignore

    _resources =           RetrieverRef(ScenarioSections.player_data2, PlayerData2.resources) # type: ignore

    _player_stances =      RetrieverRef(ScenarioSections.diplomacy, Diplomacy.player_stances) # type: ignore

    def __init__(self, struct: ScenarioSections):
        self._struct = struct
