from __future__ import annotations
from abc import ABC, abstractmethod
from typing import Literal, Any, Type

from src.generators.IncrementalGenerator import IncrementalGenerator


class ParserType(ABC):

    @classmethod
    def is_valid(cls, value: Any) -> tuple[bool, str]:
        return True, ""

    @classmethod
    @abstractmethod
    def from_generator(cls, igen: IncrementalGenerator, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,)) -> Any:
        ...

    @classmethod
    @abstractmethod
    def from_bytes(cls, bytes_: bytes, *, byteorder: Literal["big", "little"] = "little", struct_version: tuple[int, ...] = (0,)) -> Any:
        ...

    @classmethod
    @abstractmethod
    def to_bytes(cls, value: ParserType, *, byteorder: Literal["big", "little"] = "little") -> bytes:
        ...


ParserTypeObjCls = Type[ParserType] | ParserType
