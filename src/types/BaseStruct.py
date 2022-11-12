from __future__ import annotations
from typing import TYPE_CHECKING, Literal

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
    def from_generator(cls, igen: IncrementalGenerator, *, byteorder: Literal["big", "little"] = "little", file_version: tuple[int, ...] = (0, )) -> BaseStruct:
        with ignored(VersionError):
            file_version = cls.get_file_version(igen)

        instance = cls(file_version)
        for retriever in cls._retrievers:
            if not retriever.supported(instance.file_version):
                continue

            if retriever.repeat == 1:
                setattr(instance, retriever.p_name, retriever.cls.from_generator(igen))
                continue

            ls: list = [None] * retriever.repeat
            for i in range(retriever.repeat):
                ls[i] = retriever.cls.from_generator(igen)
            setattr(instance, retriever.p_name, ls)

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
        bytes_ = [b""]*len(instance._retrievers)

        for i, retriever in enumerate(instance._retrievers):
            if not retriever.supported(instance.file_version):
                continue

            if retriever.repeat == 1:
                bytes_[i] = retriever.cls.to_bytes(getattr(instance, retriever.p_name))
                continue

            ls: list[bytes] = [b""]*retriever.repeat
            for j, value in enumerate(getattr(instance, retriever.p_name)):
                ls[j] = retriever.cls.to_bytes(value)
            bytes_[i] = b"".join(ls)

        return b"".join(bytes_)

    def to_file(self, filename: str):
        with open(filename, "wb") as file:
            for retriever in self._retrievers:
                if not retriever.supported(self.file_version):
                    continue

                if retriever.repeat == 1:
                    file.write(retriever.cls.to_bytes(getattr(self, retriever.p_name)))
                    continue

                ls: list = getattr(self, retriever.p_name)

                if not len(ls) == retriever.repeat:
                    raise ValueError(f"length of {retriever.p_name!r} is not the same as {retriever.repeat = }")

                for value in ls:
                    file.write(retriever.cls.to_bytes(value))
