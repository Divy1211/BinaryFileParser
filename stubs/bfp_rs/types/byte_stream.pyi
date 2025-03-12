from typing import Type, TypeVar, Self


class ByteStream:
    """
    """

    def __new__(cls) -> Self: ...

    @classmethod
    def from_file(cls, filepath: str) -> Self: ...

    @classmethod
    def from_bytes(cls, bytes_: bytes) -> Self: ...

    def get(self, n: int) -> bytes: ...

    def peek(self, n: int) -> bytes: ...

    def remaining(self) -> bytes: ...
