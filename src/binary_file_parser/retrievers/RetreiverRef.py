from __future__ import annotations

from contextlib import suppress
from typing import Type, TypeVar, Generic

from binary_file_parser import Retriever, VersionError
from binary_file_parser.retrievers.BaseStruct import BaseStruct


T = TypeVar("T")


def ver_str(ver: tuple[int, ...]) -> str:
    return ".".join(map(str, ver))

class RetrieverRef(Generic[T]):
    def __init__(self, retriever: Retriever):
        self.retriever = retriever

    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None:
        self.name = name

    def __set__(self, instance: BaseStruct, value: T) -> None:
        with suppress(VersionError):
            setattr(instance, self.retriever.p_name, value)
            return
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {ver_str(instance.struct_version)!r}"
        )

    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]) -> RetrieverRef | T:
        if instance is None:
            return self
        with suppress(VersionError):
            return getattr(instance, self.retriever.p_name)
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {ver_str(instance.struct_version)!r}"
        )
