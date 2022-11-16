from __future__ import annotations
from typing import TYPE_CHECKING, Literal

from alive_progress import alive_bar

from src.errors.CompressionError import CompressionError
from src.errors.ParserError import ParserError
from src.errors.VersionError import VersionError
from src.types.IncrementalGenerator import IncrementalGenerator
from src.types.ParserType import ParserType

if TYPE_CHECKING:
    from src.retrievers.Retriever import Retriever


class BaseStruct(ParserType):
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

    def __init__(self, struct_version: tuple[int, ...] = (0,)):
        self.struct_version = struct_version

    @classmethod
    def get_version(cls, igen: IncrementalGenerator) -> tuple[int, ...]:
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
    def from_generator(
        cls, igen: IncrementalGenerator, *, byteorder: Literal["big", "little"] = "little",
        struct_version: tuple[int, ...] = (0,), strict: bool = False, top = False,
    ) -> BaseStruct:
        try:
            struct_version = cls.get_version(igen)
        except VersionError:
            pass

        instance = cls(struct_version)
        if top:
            with alive_bar(len(cls._retrievers), dual_line = True, title = "Reading File") as bar:
                for retriever in cls._retrievers:
                    bar.text = f"  -> {retriever.p_name.title().replace('_', ' ')}"
                    if retriever.remaining_compressed:
                        igen = IncrementalGenerator.from_bytes(cls.decompress(igen.get_remaining_bytes()))
                    retriever.from_generator(instance, igen)
                    bar()
        else:
            for retriever in cls._retrievers:
                if retriever.remaining_compressed:
                    igen = IncrementalGenerator.from_bytes(cls.decompress(igen.get_remaining_bytes()))
                retriever.from_generator(instance, igen)

        file_len = len(igen.file_content)

        if igen.progress != file_len and strict:
            raise ParserError(f"{file_len - igen.progress} bytes are left after parsing all retrievers successfully")

        return instance

    @classmethod
    def from_bytes(
        cls, bytes_: bytes, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,),
        strict = False,
    ) -> BaseStruct:
        igen = IncrementalGenerator.from_bytes(bytes_)
        return cls.from_generator(igen, struct_version = struct_version, strict = strict)

    @classmethod
    def from_file(cls, file_name: str, *, file_version: tuple[int, ...] = (0,), strict = False) -> BaseStruct:
        igen = IncrementalGenerator.from_file(file_name)
        return cls.from_generator(igen, struct_version = file_version, strict = strict, top = True)

    @classmethod
    def to_bytes(cls, instance: BaseStruct, *, byteorder: Literal["big", "little"] = "little", top = False) -> bytes:
        length = len(instance._retrievers)

        bytes_ = [b""] * length
        compress_idx = length
        if top:
            with alive_bar(len(instance._retrievers), dual_line = True, title = "Writing File") as bar:
                for i, retriever in enumerate(instance._retrievers):
                    bar.text = f"  <- {retriever.p_name.title().replace('_', ' ')}"
                    if retriever.remaining_compressed:
                        compress_idx = i
                    bytes_[i] = retriever.to_bytes(instance)
                    bar()
        else:
            for i, retriever in enumerate(instance._retrievers):
                if retriever.remaining_compressed:
                    compress_idx = i
                bytes_[i] = retriever.to_bytes(instance)

        compressed = b""
        if compress_idx != length:
            compressed = cls.compress(b"".join(bytes_[compress_idx:]))

        return b"".join(bytes_[:compress_idx]) + compressed

    def to_file(self, file_name: str):
        with open(file_name, "wb") as file:
            file.write(self.to_bytes(self, top = True))

    # todo: write val <-> data (names) to file
    # todo: write hex (decompressed) to file
    # todo: repr
    # todo: compare
    # todo: file/header/decompressed in both hex/val <-> data
    # todo: to_json
