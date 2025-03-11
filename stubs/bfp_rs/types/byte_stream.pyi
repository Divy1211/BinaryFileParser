from typing import Type, TypeVar

T = TypeVar("T", bound = "ByteStream")


class ByteStream:
    """
    """

    def __new__(cls: Type[T]) -> T: ...

    @classmethod
    def from_file(cls: Type[T], filepath: str) -> T: ...

    @classmethod
    def from_bytes(cls: Type[T], bytes_: bytes) -> T: ...

    def get(self, n: int) -> bytes: ...

    def peek(self, n: int) -> bytes: ...

    def remaining(self) -> bytes: ...
