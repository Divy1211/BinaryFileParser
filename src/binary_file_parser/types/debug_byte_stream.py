from __future__ import annotations


from binary_file_parser.logger import logger
from binary_file_parser.types.byte_stream import ByteStream


class DebugByteStream(ByteStream):
    """A stream of bytes which can be used to get or peek n number of bytes at a time"""
    __slots__ = ()
    reader_ret: str | None = None
    show_rets: set[str] = set()
    show_all: bool = False

    def get(self, n: int) -> bytes:
        bytes_ = super().get(n)
        if self.reader_ret in self.show_rets or self.show_all:
            logger.debug(f"Retriever '{self.reader_ret}' consumed {n} bytes: {bytes_!r}")
        return bytes_

    def peek(self, n: int) -> bytes:
        bytes_ = super().peek(n)
        if self.reader_ret in self.show_rets or self.show_all:
            logger.debug(f"Retriever '{self.reader_ret}' peeked {n} bytes: {bytes_!r}")
        return bytes_
