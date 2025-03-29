from typing import Self


class ByteStream:
    """
    A stream of bytes with get and peek operations. Used during deserialization
    """
    @classmethod
    def from_file(cls, filepath: str) -> Self:
        """
        Construct a ByteStream from a file.

        Args:
            filepath: The file to read

        Returns:
            A ByteStream instance
        """
        ...

    @classmethod
    def from_bytes(cls, bytes_: bytes) -> Self:
        """
        Construct a bytestream from bytes

        Args:
            bytes_: The bytes

        Returns:
            A ByteStream instance
        """
        ...

    def get(self, n: int) -> bytes:
        """
        Get bytes from the stream and seek forward

        Args:
            n: The number of bytes to get

        Raises
            OsError: Attempting to request more bytes than the stream has left

        Returns:
            bytes of the specified length
        """
        ...

    def peek(self, n: int) -> bytes:
        """
        Get bytes from the stream without seeking forward

        Args:
            n: The number of bytes to get

        Raises
            OsError: Attempting to request more bytes than the stream has left

        Returns:
            bytes of the specified length
        """
        ...

    def remaining(self) -> bytes:
        """
        Get all the remaining bytes from the stream and seek to the end

        Returns:
            All the bytes from the stream. May be ``b""`` if the stream is empty
        """
        ...
