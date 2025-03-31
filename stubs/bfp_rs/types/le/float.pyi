from bfp_rs import ByteStream, Version


class Float:
    """
    """

    @classmethod
    def to_bytes(cls, value: float) -> bytes: ...

    @classmethod
    def from_stream(cls, value: ByteStream, ver: Version = Version(0)) -> float: ...

    @classmethod
    def from_file(cls, filepath: str) -> float: ...

    @classmethod
    def from_bytes(cls, bytes_: bytes, ver: Version = Version(0)) -> float: ...

    @classmethod
    def to_file(cls, filepath: str, value: float): ...

class f32(Float):
    """
    [De]serialize a ``float`` with a width of 4 bytes
    """

class f64(Float):
    """
    [De]serialize a ``float`` with a width of 8 bytes
    """

