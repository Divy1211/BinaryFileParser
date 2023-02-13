from __future__ import annotations

from typing import Type, TypeVar, Generic

from binary_file_parser.retrievers.BaseStruct import BaseStruct


T = TypeVar("T")


class RetrieverRef(Generic[T]):
    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None:
        self.name = f"_{name}"

    def __set__(self, instance: BaseStruct, value: T) -> None:
        setattr(instance, self.name, value)

    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]) -> RetrieverRef | T:
        if instance is None:
            return self
        return getattr(instance, self.name)
