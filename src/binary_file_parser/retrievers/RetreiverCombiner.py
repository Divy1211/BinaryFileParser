from __future__ import annotations

from contextlib import suppress
from typing import Type, TypeVar, Generic

from binary_file_parser.errors import VersionError

from binary_file_parser.retrievers.BaseStruct import BaseStruct
from binary_file_parser.retrievers.Retriever import Retriever


T = TypeVar("T")


class RetrieverCombiner(Generic[T]):
    """
    Creates a single attribute to access values from multiple mutually exclusive retrievers
    """
    def __init__(self, retrievers: list[Retriever] = None) -> None:
        """
        :param retrievers: A list of retrievers to combine
        """
        self.retrievers = retrievers or []

    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None:
        self.name = name

    def __set__(self, instance: BaseStruct, value: T) -> None:
        for retriever in self.retrievers:
            with suppress(VersionError):
                setattr(instance, retriever.p_name, value)
                return
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {instance.struct_version}"
        )

    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]) -> RetrieverCombiner | T:
        if instance is None:
            return self
        for retriever in self.retrievers:
            with suppress(VersionError):
                return getattr(instance, retriever.p_name)
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {instance.struct_version}"
        )
