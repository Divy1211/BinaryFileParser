from abc import ABC, abstractmethod
from typing import Literal, Any

from src.generators.IncrementalGenerator import IncrementalGenerator


class ParserType(ABC):
    @classmethod
    @abstractmethod
    def from_generator(cls, igen: IncrementalGenerator, byteorder: Literal["big", "little"] = "little") -> Any:
        ...

    @classmethod
    @abstractmethod
    def from_bytes(cls, bytes_: bytes, byteorder: Literal["big", "little"] = "little") -> Any:
        ...

    @classmethod
    @abstractmethod
    def to_bytes(cls, value: Any, byteorder: Literal["big", "little"] = "little") -> bytes:
        ...
