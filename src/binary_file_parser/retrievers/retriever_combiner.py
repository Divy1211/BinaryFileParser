from __future__ import annotations

from contextlib import suppress
from typing import Generic, Type, TYPE_CHECKING, TypeVar

from binary_file_parser.errors import VersionError
from binary_file_parser.types.version import Version

if TYPE_CHECKING:
    from binary_file_parser.retrievers.retriever import Retriever
    from binary_file_parser.retrievers.retriever_ref import RetrieverRef
    from binary_file_parser.types.base_struct import BaseStruct


T = TypeVar("T")


class RetrieverCombiner(Generic[T]):
    """
    Creates a single attribute to access values from multiple retrievers which are mutually exclusive among different
    struct versions
    """
    def __init__(self, *retrievers: Retriever | RetrieverRef) -> None:
        """
        :param retrievers: The retrievers to combine
        """
        self.retrievers = retrievers

    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None:
        self.name = name
        # owner._add_combiner(self)

    def __set__(self, instance: BaseStruct, value: T) -> None:
        for retriever in self.retrievers:
            with suppress(VersionError):
                setattr(instance, retriever.p_name, value)
                return
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {instance.struct_ver}"
        )

    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]) -> RetrieverCombiner | T:
        if instance is None:
            return self
        for retriever in self.retrievers:
            with suppress(VersionError):
                return getattr(instance, retriever.p_name)
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {instance.struct_ver}"
        )

    # todo: fix this
    @property
    def p_name(self):
        return self.name

    def get_p_name(self, struct_ver: Version) -> str:
        """
        Find out the name of the retriever which holds the value referenced by this retriever combiner for the provided
        struct version
        :param struct_ver: The struct_ver to find the name of the holding retriever for
        """
        for retriever in self.retrievers:
            if retriever.supported(struct_ver):
                return retriever.p_name
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {struct_ver}"
        )
