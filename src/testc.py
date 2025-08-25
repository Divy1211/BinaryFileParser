from __future__ import annotations

from asp_test.sections import ScenarioSections, MapData
from bfp_rs import BaseStruct, Retriever, RetrieverRef, ret, Context
from bfp_rs.combinators import set_repeat, get, set_key
from bfp_rs.types.le import i16, bool8, Array, Array16


class Point(BaseStruct):
    x: int = Retriever(i16, default = 0)
    y: int = Retriever(i16, default = 0)
    z: int = Retriever(i16, default = 0)

class Test(BaseStruct):
    points: list[Point] = Retriever(Array16[Point], default_factory = lambda _ver: [])

a = Test()
a.points.append(p := Point(x = 10, y = 10, z = 10))
p.w = 20
print(a.points[0].w)
