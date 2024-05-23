from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever, RetrieverCombiner, Version
from binary_file_parser.types import (
    Array32, uint32,
)
from testing.sections.unit_data.scenario_player_data import ScenarioPlayerData
from testing.sections.unit_data.world_player_data import WorldPlayerData
from testing.sections.unit_data.unit import Unit


class UnitData(BaseStruct):
    @staticmethod
    def set_world_player_dat_repeat(_, instance: UnitData):
        UnitData.world_player_data.set_repeat(instance, instance.num_world_players - 1)
        UnitData.units.set_repeat(instance, instance.num_world_players)

    @staticmethod
    def set_scx_player_dat_repeat(_, instance: UnitData):
        UnitData.scenario_player_data.set_repeat(instance, instance.num_scenario_players - 1)

    @staticmethod
    def sync_num_world_players(_, instance: UnitData):
        if len(instance.units) != len(instance.world_player_data) + 1:
            raise ValueError("Number of world players does not match the number of unit lists for players")
        instance.num_world_players = len(instance.world_player_data) + 1

    @staticmethod
    def sync_num_scx_players(_, instance: UnitData):
        instance.num_scenario_players = len(instance.scenario_player_data) + 1

    # @formatter:off
    # these lists can't be Array32s because the length is -1 the num_value. yES
    num_world_players: int =                         Retriever(uint32,                                        default = 9, on_set = [set_world_player_dat_repeat], on_write = [sync_num_world_players])
    world_player_data: list[WorldPlayerData] =       Retriever(WorldPlayerData,                               default_factory = lambda sv: WorldPlayerData(sv),    repeat = 8)

    _units_aoc: list[list[Unit]] =                   Retriever(Array32[Unit],      max_ver = Version((1, 35)), default_factory = lambda _: [],                      repeat = 9)
    num_scenario_players: int =                      Retriever(uint32,                                         default = 9, on_set = [set_scx_player_dat_repeat],   on_write = [sync_num_scx_players])
    scenario_player_data: list[ScenarioPlayerData] = Retriever(ScenarioPlayerData,                             default_factory = lambda sv: ScenarioPlayerData(sv), repeat = 8)
    _units_de: list[list[Unit]] =                    Retriever(Array32[Unit],      min_ver = Version((1, 36)), default_factory = lambda _: [],                      repeat = 9)

    units: list[list[Unit]] =                        RetrieverCombiner(_units_de, _units_aoc)
    # @formatter:on

    def __init__(self, struct_ver: Version = Version((1, 47)), initialise_defaults = True, **retriever_inits):
        super().__init__(struct_ver, initialise_defaults = initialise_defaults, **retriever_inits)
