from typing import TypeVar, Generic, Any

from bfp_rs import ByteStream, Version

T = TypeVar("T")

class StackedArray(Generic[T]):
    """
    This class cannot be used on its own. Use one of its subclasses
    """

    @classmethod
    def to_bytes(cls, value: list[list[T]]) -> bytes: ...

    @classmethod
    def from_stream(cls, value: ByteStream, ver: Version = Version(0)) -> list[list[T]]: ...

    @classmethod
    def from_file(cls, filepath: str) -> list[list[T]]: ...

    @classmethod
    def from_bytes(cls, bytes_: bytes, ver: Version = Version(0)) -> list[list[T]]: ...

    @classmethod
    def to_file(cls, filepath: str, value: list[list[T]]): ...

    @classmethod
    def __class_getitem__(cls, item: Any) -> type: ...

class StackedArray8(StackedArray[T], Generic[T]):
    """
    [De]serialize a ``list[list[T]]`` whose length is indicated by a leading ``u8`` followed by that many ``u8``s
    indicating the lengths of each of the nested lists, using the syntax ``StackedArray8[T]``. The length of the outer
    list may be fixed to ``N`` using the syntax ``StackedArray8[N][T]`` which then reads ``N`` ``u8``s to determine
    the lengths of the nested lists.

    For example, ``StackedArray8[bool8]`` will read a ``u8``, followed by that many ``u8``s - the sum of these
    quantities indicates how many total ``bool8``s will be read.

    Essentially, instead of each list's length immediately preceding them, all the lengths for the sub lists are
    specified right at the start after specifying how many sub lists there are
    """

class StackedArray16(StackedArray[T], Generic[T]):
    """
    [De]serialize a ``list[list[T]]`` whose length is indicated by a leading ``u16`` followed by that many ``u16``s
    indicating the lengths of each of the nested lists, using the syntax ``StackedArray16[T]``. The length of the outer
    list may be fixed to ``N`` using the syntax ``StackedArray16[N][T]`` which then reads ``N`` ``u16``s to determine
    the lengths of the nested lists.

    For example, ``StackedArray16[bool8]`` will read a ``u16``, followed by that many ``u16``s - the sum of these
    quantities indicates how many total ``bool8``s will be read.

    Essentially, instead of each list's length immediately preceding them, all the lengths for the sub lists are
    specified right at the start after specifying how many sub lists there are
    """

class StackedArray32(StackedArray[T], Generic[T]):
    """
    [De]serialize a ``list[list[T]]`` whose length is indicated by a leading ``u32`` followed by that many ``u32``s
    indicating the lengths of each of the nested lists, using the syntax ``StackedArray32[T]``. The length of the outer
    list may be fixed to ``N`` using the syntax ``StackedArray32[N][T]`` which then reads ``N`` ``u32``s to determine
    the lengths of the nested lists.

    For example, ``StackedArray32[bool8]`` will read a ``u32``, followed by that many ``u32``s - the sum of these
    quantities indicates how many total ``bool8``s will be read.

    Essentially, instead of each list's length immediately preceding them, all the lengths for the sub lists are
    specified right at the start after specifying how many sub lists there are
    """

class StackedArray64(StackedArray[T], Generic[T]):
    """
    [De]serialize a ``list[list[T]]`` whose length is indicated by a leading ``u64`` followed by that many ``u64``s
    indicating the lengths of each of the nested lists, using the syntax ``StackedArray64[T]``. The length of the outer
    list may be fixed to ``N`` using the syntax ``StackedArray64[N][T]`` which then reads ``N`` ``u64``s to determine
    the lengths of the nested lists.

    For example, ``StackedArray64[bool8]`` will read a ``u64``, followed by that many ``u64``s - the sum of these
    quantities indicates how many total ``bool8``s will be read.

    Essentially, instead of each list's length immediately preceding them, all the lengths for the sub lists are
    specified right at the start after specifying how many sub lists there are
    """

class StackedArray128(StackedArray[T], Generic[T]):
    """
    [De]serialize a ``list[list[T]]`` whose length is indicated by a leading ``u128`` followed by that many ``u128``s
    indicating the lengths of each of the nested lists, using the syntax ``StackedArray128[T]``. The length of the outer
    list may be fixed to ``N`` using the syntax ``StackedArray128[N][T]`` which then reads ``N`` ``u128``s to determine
    the lengths of the nested lists.

    For example, ``StackedArray128[bool8]`` will read a ``u128``, followed by that many ``u128``s - the sum of these
    quantities indicates how many total ``bool8``s will be read.

    Essentially, instead of each list's length immediately preceding them, all the lengths for the sub lists are
    specified right at the start after specifying how many sub lists there are
    """

