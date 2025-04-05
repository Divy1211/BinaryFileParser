from bfp_rs import ByteStream, Version


class Bool:
    """
    """

    @classmethod
    def to_bytes(cls, value: bool) -> bytes: ...

    @classmethod
    def from_stream(cls, value: ByteStream, ver: Version = Version(0)) -> bool: ...

    @classmethod
    def from_file(cls, filepath: str) -> bool: ...

    @classmethod
    def from_bytes(cls, bytes_: bytes, ver: Version = Version(0)) -> bool: ...

    @classmethod
    def to_file(cls, filepath: str, value: bool): ...

class bool8(Bool):
    """
    [De]serialize a ``bool`` by reading a ``num: u8`` and evaluating `num != 0`
    """

class bool16(Bool):
    """
    [De]serialize a ``bool`` by reading a ``num: u16`` and evaluating `num != 0`
    """

class bool32(Bool):
    """
    [De]serialize a ``bool`` by reading a ``num: u32`` and evaluating `num != 0`
    """

class bool64(Bool):
    """
    [De]serialize a ``bool`` by reading a ``num: u64`` and evaluating `num != 0`
    """

class bool128(Bool):
    """
    [De]serialize a ``bool`` by reading a ``num: u128`` and evaluating `num != 0`
    """

