from typing import Any

from bfp_rs.diff.diff_py import Diff

class Conflict:
    """Base class for all conflict types"""


class Basic(Conflict):
    __match_args__ = ("old", "change1", "change2")

    old: Any
    change1: Diff
    change2: Diff

class NestedConflict(Conflict):
    __match_args__ = ("children", )

    children: dict[str, Conflict] | dict[int, Conflict]

    def __getitem__(self, item: Any) -> Diff:
        ...
