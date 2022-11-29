from __future__ import annotations


class ByteStream:
    """A stream of bytes which can be used to get or peek n number of bytes at a time"""
    __slots__ = "content", "progress"

    def __init__(self, content: bytes, progress: int = 0):
        """
        :param content: The content of the file in bytes
        :param progress: The number of bytes that have been read from the content
        """
        self.content: bytes = content
        self.progress = progress

    @classmethod
    def from_file(cls, filepath: str) -> ByteStream:
        """
        Create a ByteStream from file

        :param filepath: The path of the file to create the stream from
        :return: ByteStream object
        """
        with open(filepath, 'rb') as f:
            file_content = f.read()
        return cls(file_content)

    @classmethod
    def from_bytes(cls, bytes_: bytes) -> ByteStream:
        """
        Create a ByteStream from bytes

        :param bytes_:
        :return: ByteStream object
        """
        return cls(bytes_)

    def get(self, n: int) -> bytes:
        """
        Get the specified number of bytes from the stream and advance the reading position forward

        :param n: The number of bytes to return from the stream
        :return: The requested bytes
        :raises EOFError: if the number of bytes requested is greater than the remaining number of bytes in the stream
        """
        if n <= 0:
            return b''
        result = self.content[self.progress:self.progress + n]
        if not result:
            remaining = len(self.remaining())
            raise EOFError(f"End of file reached. (Requested: {n} bytes, only {remaining} left.)")
        self.progress += n
        return result

    def peek(self, n: int) -> bytes:
        """
        Get the specified number of bytes from the stream without advancing the reading position forward

        :param n: The number of bytes to return from the stream
        :return: The requested bytes
        :raises EOFError: if the number of bytes requested is greater than the remaining number of bytes in the stream
        """
        if n <= 0:
            return b''
        result = self.content[self.progress:self.progress + n]
        if not result:
            remaining = len(self.remaining())
            raise EOFError(f"End of file reached. (Requested: {n} bytes, only {remaining} left.)")
        return result

    def remaining(self) -> bytes:
        """
        Get all the bytes that are remaining in the stream

        :return: The remaining bytes in the stream
        """
        result = self.content[self.progress:]
        self.progress = len(self.content) - 1
        return result
