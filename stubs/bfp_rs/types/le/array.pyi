from typing import TypeVar, Generic, Self, Any

from bfp_rs import ByteStream, Version

T = TypeVar("T")

class Array(Generic[T]):
    """
    [De]serialize a ``list[T]`` of fixed length ``N`` using the syntax ``Array[N][T]``
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

class Array8(Array[T], Generic[T]):
    """
    [De]serialize a ``list[T]`` whose length is indicated by a leading ``u8``
    """

class Array16(Array[T], Generic[T]):
    """
    [De]serialize a ``list[T]`` whose length is indicated by a leading ``u16``
    """

class Array32(Array[T], Generic[T]):
    """
    [De]serialize a ``list[T]`` whose length is indicated by a leading ``u32``
    """

class Array64(Array[T], Generic[T]):
    """
    [De]serialize a ``list[T]`` whose length is indicated by a leading ``u64``
    """

class Array128(Array[T], Generic[T]):
    """
    [De]serialize a ``list[T]`` whose length is indicated by a leading ``u128``
    """

