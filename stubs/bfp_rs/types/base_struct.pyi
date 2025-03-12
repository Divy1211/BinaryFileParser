from typing import Any, Self

from bfp_rs import ByteStream, Version


class BaseStruct:
    """
    """
    ver: Version

    def __new__(cls, ver: Version = Version(-1), init_defaults: bool = True, **retriever_inits: Any) -> Self: ...

    @classmethod
    def from_stream(cls, stream: ByteStream, ver: Version = Version(0)) -> Self: ...

    @classmethod
    def to_bytes(cls, value: BaseStruct) -> bytes: ...

    @classmethod
    def from_bytes(cls, bytes: bytes) -> Self: ...

    @classmethod
    def from_file(cls, filepath: str, strict: bool = True) -> Self: ...

    @classmethod
    def to_file(cls, filepath: str, value: BaseStruct) -> None: ...

    @classmethod
    def _get_version(cls, stream: ByteStream, ver: Version = Version(0)) -> Version: ...

    @classmethod
    def _compress(cls, bytes_: bytes) -> bytes: ...

    @classmethod
    def _decompress(cls, bytes_: bytes) -> bytes: ...
