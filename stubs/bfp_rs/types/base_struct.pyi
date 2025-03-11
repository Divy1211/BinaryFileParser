from typing import Any, Type, TypeVar

from bfp_rs import ByteStream, Version

T = TypeVar("T", bound = "BaseStruct")


class BaseStruct:
    """
    """
    ver: Version

    def __new__(cls: Type[T], ver: Version = Version(-1), init_defaults: bool = True, **retriever_inits: Any) -> T: ...

    @classmethod
    def from_stream(cls: Type[T], stream: ByteStream, ver: Version = Version(0)) -> T: ...

    @classmethod
    def to_bytes(cls, value: BaseStruct) -> bytes: ...

    @classmethod
    def from_bytes(cls: Type[T], bytes: bytes) -> T: ...

    @classmethod
    def from_file(cls: Type[T], filepath: str, strict: bool = True) -> T: ...

    @classmethod
    def to_file(cls, filepath: str, value: BaseStruct) -> None: ...

    @classmethod
    def _get_version(cls, stream: ByteStream, ver: Version = Version(0)) -> Version: ...

    @classmethod
    def _compress(cls, bytes_: bytes) -> bytes: ...

    @classmethod
    def _decompress(cls, bytes_: bytes) -> bytes: ...
