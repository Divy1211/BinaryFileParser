from __future__ import annotations

from contextlib import suppress
from operator import attrgetter, itemgetter
from typing import Any, Callable, Type, TYPE_CHECKING, TypeVar, Generic

from binary_file_parser.errors import VersionError

if TYPE_CHECKING:
    from binary_file_parser.types.base_struct import BaseStruct
    from binary_file_parser.retrievers.retriever import Retriever
    from binary_file_parser.retrievers.retriever_combiner import RetrieverCombiner


T = TypeVar("T")


class RetrieverRef(Generic[T]):
    """
    Create a new reference to an existing retriever
    """
    def __init__(self, *retrievers: Retriever | RetrieverCombiner | int):
        """
        :param retrievers: The retrievers or the list indexes to the final property to reference
        """
        if len(retrievers) == 0:
            raise TypeError("Cannot create an empty reference")

        self.retrievers = retrievers
        self.getter: list[Callable[[BaseStruct], Any]] = []
        self.last: int | str = -1

    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None:
        self.name = name

        self.getter = [
            itemgetter(retriever) if isinstance(retriever, int) else attrgetter(retriever.p_name)
            for retriever in self.retrievers
        ]
        last_retriever = self.retrievers[-1]
        self.last = last_retriever if isinstance(last_retriever, int) else last_retriever.p_name
        # owner._add_ref(self)

    def __set__(self, instance: BaseStruct, value: T) -> None:
        item = instance._struct
        with suppress(VersionError):
            for get in self.getter[:-1]:
                item = get(item)
            if isinstance(self.last, int):
                item[self.last] = value
            else:
                setattr(item, self.last, value)
            return
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {instance.struct_ver}"
        )

    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]) -> RetrieverRef | T:
        if instance is None:
            return self
        item = instance._struct
        with suppress(VersionError):
            for get in self.getter:
                item = get(item)
            return item
        raise VersionError(
            f"{self.name!r} is not supported in your struct version {instance.struct_ver}"
        )

    # todo: fix this
    @property
    def p_name(self):
        return self.name
