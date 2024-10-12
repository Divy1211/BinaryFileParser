from __future__ import annotations

from typing import Iterable


class Version(tuple[int, ...]):
    """
    - Functions equivalently to ``tuple[int, ...]``, e.g.: ``Version((1, 2, 3))`` is exactly the same as ``(1, 2, 3)``
    - Exists solely for custom ``str()`` and ``repr()`` so ``Version((1, 2, 3))`` is printed as ``v1.2.3``
    """
    def __repr__(self):
        return f"Version({super().__repr__()})"

    def __str__(self):
        return "v"+'.'.join(map(str, self))

    @classmethod
    def from_ints(cls, *ints: int) -> Version:
        return Version(ints)

    @classmethod
    def from_str(cls, version_str: str) -> Version:
        return Version(map(int, version_str.split('.')))

    def __init__(self, tup: Iterable[int]):
        ...

    def __add__(self, other: tuple[int, ...]):
        if not isinstance(other, tuple):
            raise TypeError("Cannot add non tuple to version")
        if len(other) == 0:
            return self
        if not all(map(lambda x: isinstance(x, int), other)):
            raise TypeError("tuple must be ints to version")
        return Version(tuple(self) + other)
