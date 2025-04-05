from bfp_rs import ByteStream, Version
from bfp_rs.types.le.encodings import Encoding


class Str:
    """
    [De]serialize a string with a fixed length ``N`` using the syntax ``Str[N]``. Use the syntax
    ``Str[N]._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """
    _0: type[Str]

    @classmethod
    def to_bytes(cls, value: str) -> bytes: ...

    @classmethod
    def from_stream(cls, value: ByteStream, ver: Version = Version(0)) -> str: ...

    @classmethod
    def from_file(cls, filepath: str) -> str: ...

    @classmethod
    def from_bytes(cls, bytes_: bytes, ver: Version = Version(0)) -> str: ...

    @classmethod
    def to_file(cls, filepath: str, value: str): ...

    @classmethod
    def __class_getitem__(cls, item: int | Encoding | tuple[Encoding, Encoding]) -> type: ...


class str8(Str):
    """
    [De]serialize a string whose length is indicated by a leading ``u8``. Use the syntax
    ``str8._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """

class str16(Str):
    """
    [De]serialize a string whose length is indicated by a leading ``u16``. Use the syntax
    ``str16._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """

class str32(Str):
    """
    [De]serialize a string whose length is indicated by a leading ``u32``. Use the syntax
    ``str32._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """

class str64(Str):
    """
    [De]serialize a string whose length is indicated by a leading ``u64``. Use the syntax
    ``str64._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """

class str128(Str):
    """
    [De]serialize a string whose length is indicated by a leading ``u128``. Use the syntax
    ``str128._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """

