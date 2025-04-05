from typing import TypeVar, Generic, Any

from bfp_rs import ByteStream, Version

T = TypeVar("T")

class Option(Generic[T]):
    """
    This class cannot be used on its own. Use one of its subclasses
    """
    @classmethod
    def to_bytes(cls, value: T | None) -> bytes: ...

    @classmethod
    def from_stream(cls, value: ByteStream, ver: Version = Version(0)) -> T | None: ...

    @classmethod
    def from_file(cls, filepath: str) -> T | None: ...

    @classmethod
    def from_bytes(cls, bytes_: bytes, ver: Version = Version(0)) -> T | None: ...

    @classmethod
    def to_file(cls, filepath: str, value: T | None): ...

    @classmethod
    def __class_getitem__(cls, item: Any) -> type: ...

class Option8(Option[T], Generic[T]):
    """
    De[serialize] a type ``T`` only if the leading ``u8`` is non-zero using the syntax ``Option8[T]``
    """

class Option16(Option[T], Generic[T]):
    """
    De[serialize] a type ``T`` only if the leading ``u16`` is non-zero using the syntax ``Option16[T]``
    """

class Option32(Option[T], Generic[T]):
    """
    De[serialize] a type ``T`` only if the leading ``u32`` is non-zero using the syntax ``Option32[T]``
    """

class Option64(Option[T], Generic[T]):
    """
    De[serialize] a type ``T`` only if the leading ``u64`` is non-zero using the syntax ``Option64[T]``
    """

class Option128(Option[T], Generic[T]):
    """
    De[serialize] a type ``T`` only if the leading ``u128`` is non-zero using the syntax ``Option128[T]``
    """

