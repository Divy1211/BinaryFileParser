from typing import Any, Type

from bfp_rs.types import BaseStruct

from bfp_rs.retrievers.retriever import Retriever
from bfp_rs.retrievers.retriever_combiner import RetrieverCombiner


class RetrieverRef:
    """
    Alias another retriever, combiner, or another reference
    """

    def __new__(cls, *target: Retriever | RetrieverRef | RetrieverCombiner | int) -> RetrieverRef:
        """
        Alias another retriever, combiner, or another reference

        Args:
            *target: The retriever path to alias. This can be a sequence of retrievers/list indices starting in the
            current struct
        """
        ...

    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]) -> Any: ...

    def __set__(self, instance: BaseStruct, value: Any) -> None: ...

    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None: ...
