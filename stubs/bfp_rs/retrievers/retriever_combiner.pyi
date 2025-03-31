from typing import Any, Type

from bfp_rs.types import BaseStruct

from bfp_rs.retrievers.retriever import Retriever
from bfp_rs.retrievers.retriever_ref import RetrieverRef


class RetrieverCombiner:
    """
    Multiplexes retrievers that are mutually exclusive by version but hold the same value conceptually into a single
    property for ease of access across multi-versioned structs
    """

    def __new__(cls, *target: Retriever | RetrieverRef | RetrieverCombiner) -> RetrieverCombiner:
        """
        Create a new combiner property from the given retrievers. Note: mutual exclusivity is not checked for the
        provided retrievers, and the first supported retriever from the ones provided will be used as the "source" when
        this combiner's property is accessed

        Args:
            *target: The retrievers to group/select from
        """


    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]) -> Any: ...

    def __set__(self, instance: BaseStruct, value: Any) -> None: ...

    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None: ...
