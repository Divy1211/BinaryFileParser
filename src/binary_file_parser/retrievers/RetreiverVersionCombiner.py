from __future__ import annotations

from contextlib import suppress
from typing import Type, TypeVar, Generic

from binary_file_parser.errors import VersionError
from binary_file_parser.retrievers.BaseStruct import BaseStruct


T = TypeVar("T")


def ver_str(ver: tuple[int, ...]) -> str:
    return ".".join(map(str, ver))


class RetrieverVersionCombiner(Generic[T]):
    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None:
        self.name = name
        self.retrievers = []
        for retriever in owner._retrievers:
            if retriever.p_name.startswith(f"_{name}"):
                self.retrievers.append(retriever)

    def __set__(self, instance: BaseStruct, value: T) -> None:
        for retriever in self.retrievers:
            with suppress(VersionError):
                setattr(instance, retriever.p_name, value)
                return
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {ver_str(instance.struct_version)!r}"
        )

    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]) -> RetrieverVersionCombiner | T:
        if instance is None:
            return self
        for retriever in self.retrievers:
            with suppress(VersionError):
                return getattr(instance, retriever.p_name)
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {ver_str(instance.struct_version)!r}"
        )
