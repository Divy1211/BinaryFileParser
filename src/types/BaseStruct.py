from __future__ import annotations

from typing import TYPE_CHECKING

from alive_progress import alive_it

from src.errors.CompressionError import CompressionError
from src.errors.ParserError import ParserError
from src.errors.VersionError import VersionError
from src.types.ByteStream import ByteStream
from src.types.Parseable import Parseable

if TYPE_CHECKING:
    from src.retrievers.Retriever import Retriever


class BaseStruct(Parseable):
    __slots__ = "struct_version", "parent"

    _retrievers: list[Retriever] = []

    @classmethod
    def add_retriever(cls, retriever: Retriever):
        cls._retrievers.append(retriever)

    def __init_subclass__(cls, **kwargs):
        cls_retrievers = cls._retrievers[:]
        BaseStruct._retrievers = []
        cls._retrievers = cls_retrievers

    @classmethod
    def default(cls, struct_version: tuple[int, ...] = None):
        instance = cls() if not struct_version else cls(struct_version)
        for retriever in cls._retrievers:
            if hasattr(instance, retriever.r_name):
                setattr(instance, retriever.p_name, [retriever.default for _ in range(retriever.repeat(instance))])
                continue
            setattr(instance, retriever.p_name, retriever.default)

    def __init__(self, struct_version: tuple[int, ...] = (0,), parent: BaseStruct = None):
        super().__init__(-1)
        self.struct_version = struct_version
        self.parent = parent

    @classmethod
    def get_version(cls, stream: ByteStream) -> tuple[int, ...]:
        raise VersionError("Un-versioned File")

    @classmethod
    def decompress(cls, bytes_: bytes) -> bytes:
        raise CompressionError(
            "Unable to read object from file. "
            "A Structure with compressed section needs to implement 'decompress' classmethod."
        )

    @classmethod
    def compress(cls, bytes_: bytes) -> bytes:
        raise CompressionError(
            "Unable to write object to file. "
            "A Structure with compressed section needs to implement 'compress' classmethod."
        )

    @classmethod
    def from_stream(
        cls, stream: ByteStream, *, struct_version: tuple[int, ...] = (0,), strict: bool = False,
        show_progress: bool = False, parent: BaseStruct = None
    ) -> BaseStruct:
        try:
            struct_version = cls.get_version(stream)
        except VersionError:
            pass

        instance = cls(struct_version, parent)
        retriever_ls = cls._retrievers
        if show_progress:
            retriever_ls = alive_it(
                retriever_ls,
                dual_line = True,
                title = f"         Reading File",
                stats = False,
                finalize = lambda bar: bar.title(f"Finished Reading File")
            )

        for retriever in retriever_ls:
            if show_progress:
                retriever_ls.text = f"            -> {retriever.p_name.title().replace('_', ' ')}"
            if retriever.remaining_compressed:
                stream = ByteStream.from_bytes(cls.decompress(stream.remaining()))
            retriever.from_stream(instance, stream)

        file_len = len(stream.content)

        if stream.progress != file_len and strict:
            raise ParserError(f"{file_len - stream.progress} bytes are left after parsing all retrievers successfully")

        return instance

    @classmethod
    def from_bytes(cls, bytes_: bytes, *, struct_version: tuple[int, ...] = (0,), strict = False) -> BaseStruct:
        stream = ByteStream.from_bytes(bytes_)
        return cls.from_stream(stream, struct_version = struct_version, strict = strict)

    @classmethod
    def from_file(cls, file_name: str, *, file_version: tuple[int, ...] = (0,), strict = False) -> BaseStruct:
        stream = ByteStream.from_file(file_name)
        return cls.from_stream(stream, struct_version = file_version, strict = strict, show_progress = True)

    @classmethod
    def to_bytes(cls, instance: BaseStruct, *, show_progress = False) -> bytes:
        length = len(instance._retrievers)

        bytes_ = [b""] * length
        compress_idx = length
        retriever_ls = instance._retrievers
        if show_progress:
            retriever_ls = alive_it(
                retriever_ls,
                dual_line = True,
                title = f"         Writing File",
                stats = False,
                finalize = lambda bar: bar.title(f"Finished Writing File")
            )

        for i, retriever in enumerate(retriever_ls):
            if show_progress:
                retriever_ls.text = f"            <- {retriever.p_name.title().replace('_', ' ')}"
            if retriever.remaining_compressed:
                compress_idx = i
            bytes_[i] = retriever.to_bytes(instance)

        compressed = b""
        if compress_idx != length:
            compressed = cls.compress(b"".join(bytes_[compress_idx:]))

        return b"".join(bytes_[:compress_idx]) + compressed

    def to_file(self, file_name: str):
        with open(file_name, "wb") as file:
            file.write(self.to_bytes(self, show_progress = True))

    # todo: write val <-> data (names) to file
    # todo: write hex (decompressed) to file
    # todo: repr
    # todo: compare
    # todo: file/header/decompressed in both hex/val <-> data
    # todo: to_json
