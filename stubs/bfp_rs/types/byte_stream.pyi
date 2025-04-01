from typing import Self


class ByteStream:
    """
    A stream of bytes with get and peek operations. Used during deserialization
    """
    @classmethod
    def from_file(cls, filepath: str) -> Self:
        """
        Construct a ``ByteStream`` from a file.

        Args:
            filepath: The file to read

        Returns:
            A ``ByteStream`` instance
        """


    @classmethod
    def from_bytes(cls, bytes_: bytes) -> Self:
        """
        Construct a ``ByteStream`` from bytes

        Args:
            bytes_: The bytes

        Returns:
            A ``ByteStream`` instance
        """


    def get(self, n: int) -> bytes:
        """
        Get bytes from the stream and seek forward

        Args:
            n: The number of bytes to get

        Raises:
            OsError: Attempting to request more bytes than the stream has left

        Returns:
            A ``bytes`` string of the specified length
        """


    def peek(self, n: int) -> bytes:
        """
        Get bytes from the stream without seeking forward

        Args:
            n: The number of bytes to get

        Raises:
            OsError: Attempting to request more bytes than the stream has left

        Returns:
            A ``bytes`` string of the specified length
        """


    def remaining(self) -> bytes:
        """
        Get all the remaining bytes from the stream and seek to the end

        Returns:
            A ``bytes`` string consisting of all the bytes remaining in the stream. May be ``b""`` if the stream is
            empty
        """

    def is_empty(self) -> bool:
        """
        Checks if this stream has been fully consumed

        Returns:
            ``True`` if the stream has no more bytes to return, else ``False``
        """