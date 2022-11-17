from __future__ import annotations


class ByteStream:
    def __init__(self, name: str, content: bytes, progress: int = 0):
        self.name = name
        self.content: bytes = content
        self.progress = progress

    @classmethod
    def from_file(cls, filepath: str) -> ByteStream:
        with open(filepath, 'rb') as f:
            file_content = f.read()
        return cls(filepath, file_content)

    @classmethod
    def from_bytes(cls, bytes_: bytes) -> ByteStream:
        return cls("_bytes", bytes_)

    def get(self, n: int) -> bytes:
        if n <= 0:
            return b''
        result = self.content[self.progress:self.progress + n]
        if not result:
            remaining = len(self.remaining())
            raise EOFError(f"End of file reached. (Requested: {n} bytes, only {remaining} left.)")
        self.progress += n
        return result

    def peek(self, n: int) -> bytes:
        if n <= 0:
            return b''
        result = self.content[self.progress:self.progress + n]
        if not result:
            remaining = len(self.remaining())
            raise EOFError(f"End of file reached. (Requested: {n} bytes, only {remaining} left.)")
        return result

    def remaining(self) -> bytes:
        result = self.content[self.progress:]
        self.progress = len(self.content) - 1
        return result
