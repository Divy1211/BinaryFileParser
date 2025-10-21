from __future__ import annotations

from asp_test.sections import ScenarioSections, MapData
from bfp_rs import BaseStruct, Retriever, RetrieverRef, ret, Context, Version
from bfp_rs.combinators import set_repeat, get, set_key
from bfp_rs.types.le import i16, bool8, Array, Array16


class Point(BaseStruct):
    x: int = Retriever(i16, default = 0)
    y: int = Retriever(i16, default = 0)

    def __str__(self) -> str:
        return f"({self.x}, {self.y})"

class Test(BaseStruct):
    # points: list[Point] = Retriever(Array16[Point], default_factory = lambda _ver: [])
    points: list[Point] = Retriever(Array16[Point], default_factory = lambda _ver: [])


# t1 = Test(points = [Point(x = i, y = i) for i in range(2)])
# t2 = Test(points = [Point(x = i+1, y = i+1) for i in range(2)])
t1 = Test(points = [Point(x = i, y = i + 1) for i in range(3)])
t2 = Test(points = [Point(), Point(x = 1, y = 2), Point()])

print(t1.points, t2.points)

t1.diff(t2)
