from __future__ import annotations

from typing import Any

from asp_test.sections import ScenarioSections, MapData
from bfp_rs import BaseStruct, Retriever, RetrieverRef, ret, Context, Version
from bfp_rs.combinators import set_repeat, get, set_key
from bfp_rs.types.le import i16, bool8, Array, Array16

from bfp_rs.diff import Inserted, Diff, Deleted, Changed, NestedDiff, NestedConflict


class Point(BaseStruct):
    x: int = Retriever(i16, default = 0)
    y: int = Retriever(i16, default = 0)

    def __str__(self) -> str:
        return f"({self.x}, {self.y})"

class Test(BaseStruct):
    # points: list[Point] = Retriever(Array16[Point], default_factory = lambda _ver: [])
    points: list[Point] = Retriever(Array16[Point], default_factory = lambda _ver: [])

    def __str__(self) -> str:
        return f"Test({self.points})"

t1 = Test(points = [Point(x = 0, y = 0), Point(x = 1, y = 1)])
t2 = Test(points = [Point(x = 1, y = 0), Point(x = 1, y = 4)])
t3 = Test(points = [Point(x = 2, y = 2), Point(x = 3, y = 1), Point(x = 2, y = 2)])

t1.merge(t2, t3)
