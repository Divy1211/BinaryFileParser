from __future__ import annotations

from contextlib import suppress
from typing import Type, TypeVar, Generic

from binary_file_parser.errors import VersionError

from binary_file_parser.types.base_struct import BaseStruct
from binary_file_parser.retrievers.retriever import Retriever
from binary_file_parser.retrievers.retriever_combiner import RetrieverCombiner


T = TypeVar("T")


class RetrieverRef(Generic[T]):
    """
    Create a new reference to an existing retriever
    """
    def __init__(self, retriever: Retriever | RetrieverCombiner):
        """
        :param retriever: The retriever to reference
        """
        self.retriever = retriever

    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None:
        self.name = name
        owner._add_ref(self)

    def __set__(self, instance: BaseStruct, value: T) -> None:
        with suppress(VersionError):
            setattr(instance, self.retriever.p_name, value)
            return
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {instance.struct_ver}"
        )

    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]) -> RetrieverRef | T:
        if instance is None:
            return self
        with suppress(VersionError):
            return getattr(instance, self.retriever.p_name)
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {instance.struct_ver}"
        )
