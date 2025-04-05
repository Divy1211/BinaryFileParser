from typing import Any, Self

from bfp_rs.types.byte_stream import ByteStream
from bfp_rs.types.version import Version


class BaseStruct:
    """
    Base class for defining a binary serialization schema in the form of a struct using ``Retriever``s with optional
    support for compression and versioning allowing for conditional serialization
    """
    ver: Version
    "The version of the struct"

    def __new__(cls, ver: Version = Version(-1), init_defaults: bool = True, **retriever_inits: Any) -> Self:
        """
        Default initialise and create a new instance of this struct

        Args:
            ver: The struct version to create

            init_defaults:
                If set to false, skip initialisation of struct values from defaults. This is only useful when the values
                are filled in from another source during deserialization

            **retriever_inits:
                Specify overrides for the default values of retrievers for initialisation by name
        """


    @classmethod
    def from_stream(cls, stream: ByteStream, ver: Version = Version(0)) -> Self:
        """
        Deserialize and create an instance of this struct from a ``ByteStream`` according to the specified version

        Args:
            stream: The stream to use for deserialization
            ver: The version of the struct being deserialized

        Returns:
            An instance of this struct

        Raises:
            CompressionError: If the ``_decompress`` method is not defined and ``remaining_compressed`` is set to
                ``True`` in one of the retrievers
        """


    @classmethod
    def to_bytes(cls, value: BaseStruct) -> bytes:
        """
        Serialize this instance of this struct to bytes

        Args:
            value: The instance to serialize

        Returns:
            The byte representation of this struct

        Raises:
            CompressionError: If the ``_compress`` method is not defined and ``remaining_compressed`` is set to ``True``
                in one of the retrievers
        """


    @classmethod
    def from_bytes(cls, bytes_: bytes) -> Self:
        """
        Deserialize and create an instance of this struct from bytes

        Args:
            bytes_: The bytes to use for deserialization

        Returns:
            An instance of this struct

        Raises:
            CompressionError: If the ``_decompress`` method is not defined and ``remaining_compressed`` is set to
                ``True`` in one of the retrievers
        """


    @classmethod
    def from_file(cls, filepath: str, strict: bool = True) -> Self:
        """
        Deserialize and create an instance of this struct from the given file

        Args:
            filepath: The file to use for deserialization
            strict: Raise an error if the complete file is not consumed after deserialization is complete

        Returns:
            An instance of this struct

        Raises:
            ParsingError: When ``strict`` is set to ``True`` and the complete file is not consumed after deserialization

            CompressionError: If the ``_decompress`` method is not defined and ``remaining_compressed`` is set to
                ``True`` in one of the retrievers

        """


    @classmethod
    def to_file(cls, filepath: str, value: BaseStruct):
        """
        Serialize this instance of this struct to the given file

        Args:
            filepath: The path to write the serialized file to
            value: The instance to serialize

        Raises:
            CompressionError: If the ``_compress`` method is not defined and ``remaining_compressed`` is set to ``True``
                in one of the retrievers
        """


    @classmethod
    def _get_version(cls, stream: ByteStream, ver: Version = Version(0)) -> Version:
        """
        Called before deserialization begins. Used to determine the version of the struct

        Args:
            stream: The ByteStream to use for determining the version
            ver: The version of the parent struct (if any)

        Returns:
              The version to use for versioning this struct
        """


    @classmethod
    def _compress(cls, bytes_: bytes) -> bytes:
        """
        Used to compress all the bytes of the properties following a retriever with ``remaining_compressed`` set to
        ``True`` during serialization

        Args:
            bytes_: The bytes to compress

        Returns:
              The compressed bytes
        """


    @classmethod
    def _decompress(cls, bytes_: bytes) -> bytes:
        """
        Used to decompress all the bytes of the properties following a retriever with ``remaining_compressed`` set to
        ``True`` during deserialization

        Args:
            bytes_: The bytes to decompress

        Returns:
              The decompressed bytes
        """

