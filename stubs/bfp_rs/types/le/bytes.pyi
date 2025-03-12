from typing import Generic, TypeVar

from bfp_rs import ByteStream, Version

N = TypeVar("N", bound = int)

class Bytes(Generic[N]):
    """
    """

    @classmethod
    def to_bytes(cls, value: bytes) -> bytes: ...

    @classmethod
    def from_stream(cls, value: ByteStream, ver: Version = Version(0)) -> bytes: ...

    @classmethod
    def from_file(cls, filepath: str) -> bytes: ...

    @classmethod
    def from_bytes(cls, bytes_: bytes, ver: Version = Version(0)) -> bytes: ...

    @classmethod
    def to_file(cls, filepath: str, value: bytes): ...

    @classmethod
    def __class_getitem__(cls, item: int) -> type: ...
