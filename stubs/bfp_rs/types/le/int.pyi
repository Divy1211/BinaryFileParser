from bfp_rs import ByteStream, Version


class Int:
    """
    """

    @classmethod
    def to_bytes(cls, value: int) -> bytes: ...

    @classmethod
    def from_stream(cls, value: ByteStream, ver: Version = Version(0)) -> int: ...

    @classmethod
    def from_file(cls, filepath: str) -> int: ...

    @classmethod
    def from_bytes(cls, bytes_: bytes, ver: Version = Version(0)) -> int: ...

    @classmethod
    def to_file(cls, filepath: str, value: int): ...

class u8(Int):
    """
    [De]serialize an ``unsigned int`` with a width of 1 bytes
    """

class u16(Int):
    """
    [De]serialize an ``unsigned int`` with a width of 2 bytes
    """

class u32(Int):
    """
    [De]serialize an ``unsigned int`` with a width of 4 bytes
    """

class u64(Int):
    """
    [De]serialize an ``unsigned int`` with a width of 8 bytes
    """

class u128(Int):
    """
    [De]serialize an ``unsigned int`` with a width of 16 bytes
    """


class i8(Int):
    """
    [De]serialize an ``signed int`` with a width of 1 bytes
    """

class i16(Int):
    """
    [De]serialize an ``signed int`` with a width of 2 bytes
    """

class i32(Int):
    """
    [De]serialize an ``signed int`` with a width of 4 bytes
    """

class i64(Int):
    """
    [De]serialize an ``signed int`` with a width of 8 bytes
    """

class i128(Int):
    """
    [De]serialize an ``signed int`` with a width of 16 bytes
    """

