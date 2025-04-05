from bfp_rs import ByteStream, Version
from bfp_rs.types.le.encodings import Encoding


class StrArray:
    """
    This class cannot be used on its own. Use one of its subclasses
    """
    _0: type[StrArray]

    @classmethod
    def to_bytes(cls, value: list[str]) -> bytes: ...

    @classmethod
    def from_stream(cls, value: ByteStream, ver: Version = Version(0)) -> list[str]: ...

    @classmethod
    def from_file(cls, filepath: str) -> list[str]: ...

    @classmethod
    def from_bytes(cls, bytes_: bytes, ver: Version = Version(0)) -> list[str]: ...

    @classmethod
    def to_file(cls, filepath: str, value: list[str]): ...

    @classmethod
    def __class_getitem__(cls, item: int | Encoding | tuple[Encoding, Encoding]) -> type: ...


class str_array8(StrArray):
    """
    [De]serialize a ``list[str]`` whose length is indicated by a leading ``u8`` followed by that many ``u8``s
    indicating the length of each individual string. The length of the list may be fixed to ``N`` using the syntax
    ``str_array8._0[N]`` which then reads ``N`` ``u8``s to determine the lengths of each string. Use the syntax
    ``str_array8._0[Encoding.UTF8, Encoding.ASCII]`` or ``str_array8._0[N]._0[Encoding.UTF8, Encoding.ASCII]`` to
    specify a main and optionally a second fallback encoding (UTF8 and ASCII are defaults)
    """

class str_array16(StrArray):
    """
    [De]serialize a ``list[str]`` whose length is indicated by a leading ``u16`` followed by that many ``u16``s
    indicating the length of each individual string. The length of the list may be fixed to ``N`` using the syntax
    ``str_array16._0[N]`` which then reads ``N`` ``u16``s to determine the lengths of each string. Use the syntax
    ``str_array16._0[Encoding.UTF8, Encoding.ASCII]`` or ``str_array16._0[N]._0[Encoding.UTF8, Encoding.ASCII]`` to
    specify a main and optionally a second fallback encoding (UTF8 and ASCII are defaults)
    """

class str_array32(StrArray):
    """
    [De]serialize a ``list[str]`` whose length is indicated by a leading ``u32`` followed by that many ``u32``s
    indicating the length of each individual string. The length of the list may be fixed to ``N`` using the syntax
    ``str_array32._0[N]`` which then reads ``N`` ``u32``s to determine the lengths of each string. Use the syntax
    ``str_array32._0[Encoding.UTF8, Encoding.ASCII]`` or ``str_array32._0[N]._0[Encoding.UTF8, Encoding.ASCII]`` to
    specify a main and optionally a second fallback encoding (UTF8 and ASCII are defaults)
    """

class str_array64(StrArray):
    """
    [De]serialize a ``list[str]`` whose length is indicated by a leading ``u64`` followed by that many ``u64``s
    indicating the length of each individual string. The length of the list may be fixed to ``N`` using the syntax
    ``str_array64._0[N]`` which then reads ``N`` ``u64``s to determine the lengths of each string. Use the syntax
    ``str_array64._0[Encoding.UTF8, Encoding.ASCII]`` or ``str_array64._0[N]._0[Encoding.UTF8, Encoding.ASCII]`` to
    specify a main and optionally a second fallback encoding (UTF8 and ASCII are defaults)
    """

class str_array128(StrArray):
    """
    [De]serialize a ``list[str]`` whose length is indicated by a leading ``u128`` followed by that many ``u128``s
    indicating the length of each individual string. The length of the list may be fixed to ``N`` using the syntax
    ``str_array128._0[N]`` which then reads ``N`` ``u128``s to determine the lengths of each string. Use the syntax
    ``str_array128._0[Encoding.UTF8, Encoding.ASCII]`` or ``str_array128._0[N]._0[Encoding.UTF8, Encoding.ASCII]`` to
    specify a main and optionally a second fallback encoding (UTF8 and ASCII are defaults)
    """

