from __future__ import annotations

import zlib
from contextlib import suppress
from os import path

from binary_file_parser import BaseStruct, ByteStream, Retriever, Version, VersionError

from tests.sections.file_data import FileData
from tests.sections.file_header import FileHeader
from tests.sections.map_data import MapData
from tests.sections.scx_versions import DE_LATEST
from tests.sections.settings import Settings
from tests.sections.trigger_data import TriggerData
from tests.sections.unit_data import UnitData


class ScenarioSections(BaseStruct):
    @staticmethod
    def sync_script_file_path(_, instance: ScenarioSections):
        with suppress(VersionError):
            instance.file_data.script_file_path = (
                instance.settings.options.script_name + ".xs" if instance.settings.options.script_name else ""
            )

    @staticmethod
    def sync_num_triggers(_, instance: ScenarioSections):
        with suppress(VersionError):
            instance.file_header.num_triggers = len(instance.trigger_data.triggers)
        with suppress(VersionError):
            instance.settings.options.num_triggers = len(instance.trigger_data.triggers)

    @staticmethod
    def sync_resources(_, instance: ScenarioSections):
        with suppress(VersionError):
            for i in range(8):
                instance.unit_data.world_player_data[i].food = instance.settings.player_options.starting_resources[i].food
                instance.unit_data.world_player_data[i].wood = instance.settings.player_options.starting_resources[i].wood
                instance.unit_data.world_player_data[i].stone = instance.settings.player_options.starting_resources[i].stone
                instance.unit_data.world_player_data[i].gold = instance.settings.player_options.starting_resources[i].gold
                instance.unit_data.world_player_data[i].ore_x = instance.settings.player_options.starting_resources[i].ore_x
                instance.unit_data.world_player_data[i].trade_goods = instance.settings.player_options.starting_resources[i].trade_goods

    # @formatter:off
    file_header: FileHeader =       Retriever(FileHeader,                                  default_factory = FileHeader, on_write = [sync_num_triggers])
    settings: Settings =            Retriever(Settings,                                    default_factory = Settings,   remaining_compressed = True)
    map_data: MapData =             Retriever(MapData,                                     default_factory = MapData)
    unit_data: UnitData =           Retriever(UnitData,                                    default_factory = UnitData,   on_write = [sync_resources])
    trigger_data: TriggerData =     Retriever(TriggerData,     min_ver = Version((1, 14)), default_factory = TriggerData)
    file_data: FileData =           Retriever(FileData,        min_ver = Version((1, 17)), default_factory = FileData,   on_write = [sync_script_file_path])
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
            self.settings.data_header.file_name = path.basename(file_name)
        super()._to_file(file_name)

    def __init__(self, struct_ver: Version = DE_LATEST, initialise_defaults = True, **retriever_inits):
        # todo: correctly initialise struct_ver `from_default` for all self versioned structs
        #  for default values that are different across different versions, use default_factory
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)

        from tests.managers import MessageManager

        self.message_manager = MessageManager(self)
