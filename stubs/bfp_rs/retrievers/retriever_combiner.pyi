from typing import Any, Type

from bfp_rs import Retriever, BaseStruct, RetrieverRef


class RetrieverCombiner:
    """
    """

    def __new__(cls, *target: Retriever | RetrieverRef | RetrieverCombiner | int) -> RetrieverCombiner: ...

    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]) -> Any: ...

    def __set__(self, instance: BaseStruct, value: Any) -> None: ...

    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None: ...
