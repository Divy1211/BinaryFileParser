from bfp_rs import ByteStream, Version
from bfp_rs.types.le.encodings import Encoding


class NtStr:
    """
    [De]serialize a null terminated string with a fixed length ``N`` using the syntax ``NtStr[N]``. Use the syntax
    ``NtStr[N]._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """
    _0: type[NtStr]

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


class c_str(NtStr):
    """
    A C style null terminated string. Use the syntax
    ``c_str._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """

class nt_str8(NtStr):
    """
    [De]serialize a null terminated string whose length is also indicated by a leading ``u8``. Use the syntax
    ``nt_str8._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """

class nt_str16(NtStr):
    """
    [De]serialize a null terminated string whose length is also indicated by a leading ``u16``. Use the syntax
    ``nt_str16._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """

class nt_str32(NtStr):
    """
    [De]serialize a null terminated string whose length is also indicated by a leading ``u32``. Use the syntax
    ``nt_str32._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """

class nt_str64(NtStr):
    """
    [De]serialize a null terminated string whose length is also indicated by a leading ``u64``. Use the syntax
    ``nt_str64._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """

class nt_str128(NtStr):
    """
    [De]serialize a null terminated string whose length is also indicated by a leading ``u128``. Use the syntax
    ``nt_str128._0[Encoding.UTF8, Encoding.ASCII]`` to specify a main and optionally a second fallback encoding
    (UTF8 and ASCII are defaults)
    """

