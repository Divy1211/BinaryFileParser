from typing import TypeVar, Generic, Self

from bfp_rs import ByteStream, Version

T = TypeVar("T")

class Array(Generic[T]):
    """
    [De]Serialize bytes into a ``list[T]`` of fixed length ``N`` using the syntax ``Array[N][T]``
    """

    @classmethod
    def from_stream(cls, stream: ByteStream, ver: Version = Version(0)) -> list[T]:
        """
        Deserialize and create a ``list[T]`` from a ``ByteStream`` according to the specified version

        Args:
            stream: The stream to use for deserialization
            ver: The version of the list being deserialized

        Returns:
            A ``list[T]``
        """
        ...

    @classmethod
    def to_bytes(cls, value: list[T]) -> bytes:
        """
        Serialize this instance of this struct to bytes

        Args:
            value: The instance to serialize

        Returns:
            The byte representation of this struct
        """
        ...

    @classmethod
    def from_bytes(cls, bytes_: bytes) -> Self:
        """
        Deserialize and create a ``list[T]`` from bytes

        Args:
            bytes_: The bytes to use for deserialization

        Returns:
            A ``list[T]``
        """
        ...

    @classmethod
    def from_file(cls, filepath: str, strict: bool = True) -> Self:
        """
        Deserialize and create a ``list[T]`` from the given file

        Args:
            filepath: The file to use for deserialization
            strict: Raise an error if the complete file is not consumed after deserialization is complete

        Returns:
            A ``list[T]``

        Raises:
            ParsingError: When ``strict`` is set to ``True`` and the complete file is not consumed after deserialization
        """
        ...

    @classmethod
    def to_file(cls, filepath: str, value: list[T]):
        """
        Serialize this instance of this struct to the given file

        Args:
            filepath: The path to write the serialized file to
            value: The instance to serialize
        """
        ...

    @classmethod
    def __class_getitem__(cls, item: int) -> type: ...

class Array8(Array[T], Generic[T]):
    """
    [De]Serialize a ``list[T]`` with the length set by the leading bytes interpreted as a ``u8``.
    """
    ...
class Array16(Array[T], Generic[T]):
    """
    [De]Serialize a ``list[T]`` with the length set by the leading bytes interpreted as a ``u16``.
    """
    ...
class Array32(Array[T], Generic[T]):
    """
    [De]Serialize a ``list[T]`` with the length set by the leading bytes interpreted as a ``u32``.
    """
    ...
class Array64(Array[T], Generic[T]):
    """
    [De]Serialize a ``list[T]`` with the length set by the leading bytes interpreted as a ``u64``.
    """
    ...
class Array128(Array[T], Generic[T]):
    """
    [De]Serialize a ``list[T]`` with the length set by the leading bytes interpreted as a ``u128``.
    """
    ...
