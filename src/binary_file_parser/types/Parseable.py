from abc import ABC, abstractmethod
from typing import TypeVar

from binary_file_parser.types.ByteStream import ByteStream
from binary_file_parser.utils import Version

T = TypeVar("T")
class Parseable(ABC):
    __slots__ = "size"

    def __init__(self, size: int):
        self.size = size

    @staticmethod
    def is_valid(value: T) -> tuple[bool, str]:
        """
        Unused atm
        :param value:
        :return:
        """
        return True, ""

    @property
    def is_struct(self) -> bool:
        """
        If a class similar to BaseStruct implements the Parsable interface, this method must be overriden to return True

        :return: True if the object has multiple retrievers, False otherwise
        """
        return False

    @property
    def is_iterable(self) -> bool:
        """
        If the datatype is an iterator (container)

        :return: True if the object is an iterator , False otherwise
        """
        return False

    @abstractmethod
    def from_stream(self, stream: ByteStream, *, struct_ver: Version = Version((0,))) -> T:
        ...

    @abstractmethod
    def from_bytes(self, bytes_: bytes, *, struct_ver: Version = Version((0,))) -> T:
        ...

    @abstractmethod
    def to_bytes(self, value: T) -> bytes:
        ...
