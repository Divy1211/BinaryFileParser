from typing import Any, Type

from bfp_rs.types import BaseStruct

from bfp_rs.retrievers.retriever import Retriever
from bfp_rs.retrievers.retriever_combiner import RetrieverCombiner


class RetrieverRef:
    """
    Aliases another retriever, combiner, or another reference for flattening potentially nested struct properties to
    help with providing a more coherent API
    """

    def __new__(cls, *target: Retriever | RetrieverRef | RetrieverCombiner | int) -> RetrieverRef:
        """
        Create a new property which aliases the retriever property at the given path.

        Args:
            *target: The retriever path to alias. This can be a sequence of retrievers/list indices starting in the
                current struct
        """


    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]) -> Any: ...

    def __set__(self, instance: BaseStruct, value: Any) -> None: ...

    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None: ...
