from typing import Type, TypeVar, Generic

from bfp_rs import ByteStream, Version, BaseStruct
from bfp_rs.types.le import Option

T = TypeVar("T", bound = BaseStruct | Option)

class StackedAttrArray(Generic[T]):
    """
    [De]serialize a ``list[T]`` of fixed length ``N`` using the syntax ``StackedAttrArray[N][T]``, where ``T`` can only
    be one of:

    - ``BaseStruct``: each sub property of the struct is [de]serialized as its own list in order. For example, a
      ``StackedAttrArray[Point2D]`` will read a length followed by that many X values, followed by the same number of Y
      values (assuming ``Point2D`` has only two properties X and Y)
    - ``OptionX[S]``: all the ``uX``s are [de]serialized as their own list followed by as many objects
      of type ``S`` as non-zero ``uX``s. All the indices with a zero ``uX`` are set to ``None``
    """

    @classmethod
    def to_bytes(cls, value: list[T]) -> bytes: ...

    @classmethod
    def from_stream(cls, value: ByteStream, ver: Version = Version(0)) -> list[T]: ...

    @classmethod
    def from_file(cls, filepath: str) -> list[T]: ...

    @classmethod
    def from_bytes(cls, bytes_: bytes, ver: Version = Version(0)) -> list[T]: ...

    @classmethod
    def to_file(cls, filepath: str, value: list[T]): ...

    @classmethod
    def __class_getitem__(cls, item: Type[BaseStruct] | Type[Option]) -> type: ...

class StackedAttrArray8(StackedAttrArray[T], Generic[T]):
    """
    [De]serialize a ``list[T]`` whose length is indicated by a leading ``u8`` using the syntax
    ``StackedAttrArray8[T]``, where ``T`` can only be one of:

    - ``BaseStruct``: each sub property of the struct is [de]serialized as its own list in order. For example, a
      ``StackedAttrArray8[Point2D]`` will read a length followed by that many X values, followed by the same number of Y
      values (assuming ``Point2D`` has only two properties X and Y)
    - ``OptionX[S]``: all the ``uX``s are [de]serialized as their own list followed by as many objects
      of type ``S`` as non-zero ``uX``s. All the indices with a zero ``uX`` are set to ``None``
    """

class StackedAttrArray16(StackedAttrArray[T], Generic[T]):
    """
    [De]serialize a ``list[T]`` whose length is indicated by a leading ``u16`` using the syntax
    ``StackedAttrArray16[T]``, where ``T`` can only be one of:

    - ``BaseStruct``: each sub property of the struct is [de]serialized as its own list in order. For example, a
      ``StackedAttrArray16[Point2D]`` will read a length followed by that many X values, followed by the same number of Y
      values (assuming ``Point2D`` has only two properties X and Y)
    - ``OptionX[S]``: all the ``uX``s are [de]serialized as their own list followed by as many objects
      of type ``S`` as non-zero ``uX``s. All the indices with a zero ``uX`` are set to ``None``
    """

class StackedAttrArray32(StackedAttrArray[T], Generic[T]):
    """
    [De]serialize a ``list[T]`` whose length is indicated by a leading ``u32`` using the syntax
    ``StackedAttrArray32[T]``, where ``T`` can only be one of:

    - ``BaseStruct``: each sub property of the struct is [de]serialized as its own list in order. For example, a
      ``StackedAttrArray32[Point2D]`` will read a length followed by that many X values, followed by the same number of Y
      values (assuming ``Point2D`` has only two properties X and Y)
    - ``OptionX[S]``: all the ``uX``s are [de]serialized as their own list followed by as many objects
      of type ``S`` as non-zero ``uX``s. All the indices with a zero ``uX`` are set to ``None``
    """

class StackedAttrArray64(StackedAttrArray[T], Generic[T]):
    """
    [De]serialize a ``list[T]`` whose length is indicated by a leading ``u64`` using the syntax
    ``StackedAttrArray64[T]``, where ``T`` can only be one of:

    - ``BaseStruct``: each sub property of the struct is [de]serialized as its own list in order. For example, a
      ``StackedAttrArray64[Point2D]`` will read a length followed by that many X values, followed by the same number of Y
      values (assuming ``Point2D`` has only two properties X and Y)
    - ``OptionX[S]``: all the ``uX``s are [de]serialized as their own list followed by as many objects
      of type ``S`` as non-zero ``uX``s. All the indices with a zero ``uX`` are set to ``None``
    """

class StackedAttrArray128(StackedAttrArray[T], Generic[T]):
    """
    [De]serialize a ``list[T]`` whose length is indicated by a leading ``u128`` using the syntax
    ``StackedAttrArray128[T]``, where ``T`` can only be one of:

    - ``BaseStruct``: each sub property of the struct is [de]serialized as its own list in order. For example, a
      ``StackedAttrArray128[Point2D]`` will read a length followed by that many X values, followed by the same number of Y
      values (assuming ``Point2D`` has only two properties X and Y)
    - ``OptionX[S]``: all the ``uX``s are [de]serialized as their own list followed by as many objects
      of type ``S`` as non-zero ``uX``s. All the indices with a zero ``uX`` are set to ``None``
    """

