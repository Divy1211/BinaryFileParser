from __future__ import annotations
from typing import TYPE_CHECKING

from src.generators.IncrementalGenerator import IncrementalGenerator

if TYPE_CHECKING:
    from src.retrievers.Retriever import Retriever


class BaseSection:
    _retrievers: list[Retriever] = []

    @classmethod
    def add_retriever(cls, retriever: Retriever):
        cls._retrievers.append(retriever)

    def __init__(self, igen: IncrementalGenerator, file_version: tuple[int, int]):
        for retriever in self._retrievers:
            retriever.ver = file_version
            if not retriever.supported:
                continue
            setattr(self, retriever.s_name, retriever.cls.from_generator(igen))

    def write_to_file(self, filename: str):
        with open(filename, "wb") as file:
            for retriever in self._retrievers:
                if not retriever.supported:
                    continue
                file.write(retriever.cls.to_bytes(getattr(self, retriever.s_name)))
