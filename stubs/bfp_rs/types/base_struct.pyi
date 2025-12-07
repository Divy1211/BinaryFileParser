from typing import Any, Self

from bfp_rs.diff import Diff, Conflict
from bfp_rs.retrievers import Retriever
from bfp_rs.types.byte_stream import ByteStream
from bfp_rs.types.context import Context
from bfp_rs.types.version import Version


class BaseStruct:
    """
    Base class for defining a binary serialization schema in the form of a struct using ``Retriever``s with optional
    support for compression and versioning allowing for conditional serialization
    """
    ver: Version
    "The version of the struct"

    @classmethod
    def retrievers(cls) -> list[Retriever]:
        ...

    def __new__(cls, *args, ver: Version = Version(-1), ctx: Context = Context(), init_defaults: bool = True, **retriever_inits: Any) -> Self:
        """
        Default initialise and create a new instance of this struct

        Args:
            args: generic args receiver
            ver: The struct version to create
            ctx: Stores ctx key/values used by on_read/on_write combinators

            init_defaults:
                If set to false, skip initialisation of struct values from defaults. This is only useful when the values
                are filled in from another source during deserialization

            **retriever_inits:
                Specify overrides for the default values of retrievers for initialisation by name
        """

    def __reconstruct__(self):
        """
        An initialization method that is called when externally loaded structs are accessed for the first time, since
        __init__ is skipped.

        Note: This method is not called when structs are created directly by code. If required, __init__ should call
              this function.
        """

    @classmethod
    def from_base(cls, value: BaseStruct) -> Self:
        """
        Aliases the data in the base class without copying it.

        Args:
            value: The base struct instance to alias

        Returns:
            An instance of this struct
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


    def to_bytes(self) -> bytes:
        """
        Serialize this struct to bytes

        Returns:
            The byte representation of this struct

        Raises:
            CompressionError: If the ``_compress`` method is not defined and ``remaining_compressed`` is set to ``True``
                in one of the retrievers
        """


    @classmethod
    def from_bytes(cls, bytes_: bytes, ver: Version = Version(0), strict: bool = False) -> Self:
        """
        Deserialize and create an instance of this struct from bytes

        Args:
            bytes_: The bytes to use for deserialization
            ver: The version whose bytes are being deserialized
            strict: Raise an error if the complete file is not consumed after deserialization is complete

        Returns:
            An instance of this struct

        Raises:
            CompressionError: If the ``_decompress`` method is not defined and ``remaining_compressed`` is set to
                ``True`` in one of the retrievers
        """


    @classmethod
    def from_file(cls, filepath: str, ver: Version = Version(0), strict: bool = True) -> Self:
        """
        Deserialize and create an instance of this struct from the given file

        Args:
            filepath: The file to use for deserialization
            ver: The version whose bytes are being deserialized
            strict: Raise an error if the complete file is not consumed after deserialization is complete

        Returns:
            An instance of this struct

        Raises:
            ParsingError: When ``strict`` is set to ``True`` and the complete file is not consumed after deserialization

            CompressionError: If the ``_decompress`` method is not defined and ``remaining_compressed`` is set to
                ``True`` in one of the retrievers

        """

    def to_file(self, filepath: str):
        """
        Serialize this struct to the given file

        Args:
            filepath: The path to write the serialized file to

        Raises:
            CompressionError: If the ``_compress`` method is not defined and ``remaining_compressed`` is set to ``True``
                in one of the retrievers
        """

    def to_json(self, filepath: str):
        """
        Serialize this struct to the given file in JSON.

        Args:
            filepath: The path to write the serialized file to
        """

    @classmethod
    def from_json(cls, filepath: str) -> Self:
        """
        Deserialize and create an instance of this struct from the given JSON file

        Args:
            filepath: The file to use for deserialization

        Returns:
            An instance of this struct

        Raises:
            ValueError: For JSONs which do not comply with the BaseStruct schema
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

    def diff(self, other: BaseStruct) -> dict[str, Diff]:
        """
        Returns a dictionary with retriever names that are different from self in other as keys, and the respective
        changes as values
        """

    def merge(self, branch1: BaseStruct, branch2: BaseStruct) -> dict[str, Conflict]:
        """
        Safe merges the changes of branch1 and branch2 into self

        Returns:
            A dictionary with retriever names that are modified in both branches, resulting in a conflict
        """
