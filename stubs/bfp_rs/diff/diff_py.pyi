from typing import Any

class Diff:
    """Base class for all diff types"""

class Inserted(Diff):
    __match_args__ = ("value",)

    value: Any

class Deleted(Diff):
    __match_args__ = ("value",)

    value: Any

class Changed(Diff):
    __match_args__ = ("old", "new")

    old: Any
    new: Any

class NestedDiff(Diff):
    __match_args__ = ("nested",)

    nested: dict[str, Diff] | dict[int, Diff]
