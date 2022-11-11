from __future__ import annotations
from typing import TYPE_CHECKING

from src.generators.IncrementalGenerator import IncrementalGenerator

if TYPE_CHECKING:
    from src.retrievers.Retriever import Retriever


class BaseSection:
    __slots__ = "file_version",
    
    _retrievers: list[Retriever] = []

    @classmethod
    def add_retriever(cls, retriever: Retriever):
        cls._retrievers.append(retriever)

    def __init__(self, igen: IncrementalGenerator, file_version: tuple[int] = (0,)):
        self.file_version = file_version

        for retriever in self._retrievers:
            if not retriever.supported(self.file_version):
                continue

            if retriever.repeat == 1:
                setattr(self, retriever.s_name, retriever.cls.from_generator(igen))
                continue

            ls: list = [None]*retriever.repeat
            for i in range(retriever.repeat):
                ls[i] = retriever.cls.from_generator(igen)
            setattr(self, retriever.s_name, ls)

    def to_bytes(self) -> bytes:
        bytes_ = [b""]*len(self._retrievers)

        for i, retriever in enumerate(self._retrievers):
            if not retriever.supported(self.file_version):
                continue

            if retriever.repeat == 1:
                bytes_[i] = retriever.cls.to_bytes(getattr(self, retriever.s_name))
                continue

            ls: list[bytes] = [b""]*retriever.repeat
            for j, value in enumerate(getattr(self, retriever.s_name)):
                ls[j] = retriever.cls.to_bytes(value)
            bytes_[i] = b"".join(ls)

        return b"".join(bytes_)

    def write_to_file(self, filename: str):
        with open(filename, "wb") as file:
            for retriever in self._retrievers:
                if not retriever.supported(self.file_version):
                    continue

                if retriever.repeat == 1:
                    file.write(retriever.cls.to_bytes(getattr(self, retriever.s_name)))
                    continue

                ls: list = getattr(self, retriever.s_name)

                if not len(ls) == retriever.repeat:
                    raise ValueError(f"length of {retriever.p_name!r} is not the same as {retriever.repeat = }")

                for value in ls:
                    file.write(retriever.cls.to_bytes(value))
