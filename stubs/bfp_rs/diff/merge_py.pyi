from typing import Any

class Conflict:
    """Base class for all conflict types"""


class Basic(Conflict):
    __match_args__ = ("old", "change1", "change2")

    old: Any
    change1: Any
    change2: Any

class NestedConflict(Conflict):
    __match_args__ = ("nested", )

    nested: dict[str, Conflict] | dict[int, Conflict]
