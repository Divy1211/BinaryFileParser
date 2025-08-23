from __future__ import annotations

from asp_test.sections import ScenarioSections, MapData
from bfp_rs import BaseStruct, Retriever, RetrieverRef, ret, Context
from bfp_rs.combinators import set_repeat, get, set_key
from bfp_rs.types.le import i16, bool8, Array


class MockSS(ScenarioSections):
    map_data: MapData = Retriever(MapData, default_factory = lambda ver: MapData(ver, width = 2, height = 2))

a = MockSS()
print(len(a.map_data.terrain_tiles))

scx = ScenarioSections()
print(len(scx.map_data.terrain_tiles))
