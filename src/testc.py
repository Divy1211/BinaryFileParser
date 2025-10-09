from __future__ import annotations

from asp_test.sections import ScenarioSections, MapData
from bfp_rs import BaseStruct, Retriever, RetrieverRef, ret, Context, Version
from bfp_rs.combinators import set_repeat, get, set_key
from bfp_rs.types.le import i16, bool8, Array, Array16

class Sub(BaseStruct):
    __default_ver__ = Version(1)

    u: int = Retriever(i16, default = 0)
    v: int = Retriever(i16, min_ver = Version(1), default = 0)
    w: int = Retriever(i16, max_ver = Version(0), default = 0)

class Point(BaseStruct):
    x: int = Retriever(i16, default = 0)
    y: int = Retriever(i16, default = 0)
    z: int = Retriever(i16, default = 0)
    sub: Sub = Retriever(Sub, default_factory = Sub)

class Test(BaseStruct):
    points: list[Point] = Retriever(Array16[Point], default_factory = lambda _ver: [])

p1 = Point(x = 1, y = 2, z = 3, sub = Sub(u = 10, v = 20))
p2 = Point(x = 3, y = 2, z = 3, sub = Sub(ver = Version(0), u = 10, w = 30))

p1.diff(p2)
