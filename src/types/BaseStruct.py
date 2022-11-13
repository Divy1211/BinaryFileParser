from __future__ import annotations
from typing import TYPE_CHECKING, Literal

from src.errors.CompressionError import CompressionError
from src.errors.VersionError import VersionError
from src.generators.IncrementalGenerator import IncrementalGenerator
from src.types.ParserType import ParserType
from src.utils import ignored

if TYPE_CHECKING:
    from src.retrievers.Retriever import Retriever


class BaseStruct(ParserType):
    __slots__ = "file_version",

    _retrievers: list[Retriever] = []

    @classmethod
    def add_retriever(cls, retriever: Retriever):
        cls._retrievers.append(retriever)

    def __init_subclass__(cls, **kwargs):
        cls_retrievers = cls._retrievers[:]
        BaseStruct._retrievers = []
        cls._retrievers = cls_retrievers

    def __init__(self, file_version: tuple[int, ...] = (0,)):
        self.file_version = file_version

    @classmethod
    def get_file_version(cls, igen: IncrementalGenerator) -> tuple[int, ...]:
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
    def from_generator(cls, igen: IncrementalGenerator, *, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> BaseStruct:
        with ignored(VersionError):
            file_version = cls.get_file_version(igen)

        instance = cls(file_version)
        for retriever in cls._retrievers:
            if retriever.remaining_compressed:
                igen = IncrementalGenerator.from_bytes(cls.decompress(igen.get_remaining_bytes()))
            retriever.from_generator(instance, igen)

        return instance

    @classmethod
    def from_bytes(cls, bytes_: bytes, *, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> BaseStruct:
        igen = IncrementalGenerator.from_bytes(bytes_)
        return cls.from_generator(igen, file_version=file_version)

    @classmethod
    def from_file(cls, filename: str, *, file_version: tuple[int, ...] = (0, )) -> BaseStruct:
        igen = IncrementalGenerator.from_file(filename)
        return cls.from_generator(igen, file_version=file_version)

    @classmethod
    def to_bytes(cls, instance: BaseStruct, *, byteorder: Literal["big", "little"] = "little") -> bytes:
        length = len(instance._retrievers)

        bytes_ = [b""]*length
        compress_idx = length
        for i, retriever in enumerate(instance._retrievers):
            if retriever.remaining_compressed:
                compress_idx = i
            bytes_[i] = retriever.to_bytes(instance)

        compressed = b""
        if compress_idx != length:
            compressed = cls.compress(b"".join(bytes_[compress_idx:]))

        return b"".join(bytes_[:compress_idx])+compressed

    def to_file(self, filename: str):
        with open(filename, "wb") as file:
            file.write(self.to_bytes(self))
