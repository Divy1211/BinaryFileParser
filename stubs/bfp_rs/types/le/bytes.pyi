from typing import Generic, TypeVar

from bfp_rs import ByteStream, Version

N = TypeVar("N", bound = int)

class Bytes(Generic[N]):
    """
    [De]serialize a raw byte string of length ``N`` using the syntax ``Bytes[N]``
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

class void(Bytes[0]):
    """
    An alias for ``Bytes[0]``. Effectively a no-op but useful for adding retriever related hooks right at the beginning
    of a struct
    """

