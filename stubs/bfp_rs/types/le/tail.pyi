from typing import TypeVar, Generic, Self, Any

from bfp_rs import ByteStream, Version

T = TypeVar("T")

class Tail(Generic[T]):
    """
    [De]serialize a ``list[T]`` until the end of the byte stream is reached using the syntax ``Tail[T]``
    """

    @classmethod
    def from_stream(cls, stream: ByteStream, ver: Version = Version(0)) -> list[T]: ...

    @classmethod
    def to_bytes(cls, value: list[T]) -> bytes: ...

    @classmethod
    def from_bytes(cls, bytes_: bytes) -> Self: ...

    @classmethod
    def from_file(cls, filepath: str, strict: bool = True) -> Self: ...

    @classmethod
    def to_file(cls, filepath: str, value: list[T]): ...

    @classmethod
    def __class_getitem__(cls, item: Any) -> type: ...
