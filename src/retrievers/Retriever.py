from __future__ import annotations


from abc import ABCMeta
from typing import Type, Any, Callable, TypeVar

from src.errors.VersionError import VersionError
from src.retrievers.MapValidate import MapValidate
from src.types.BaseStruct import BaseStruct
from src.types.ByteStream import ByteStream
from src.types.Parseable import Parseable


def ver_str(ver: tuple[int]) -> str:
    return ".".join(map(str, ver))


T = TypeVar("T")
RetrieverSub = TypeVar("RetrieverSub", bound = "Retriever")
BaseStructSub = TypeVar("BaseStructSub", bound = BaseStruct)


class Retriever(MapValidate):
    __slots__ = "dtype", "min_ver", "max_ver", "default", "_repeat", "remaining_compressed", "on_read", "on_write"

    def __init__(
        self,
        dtype: Parseable | Type[Parseable],
        min_ver: tuple[int] = (-1,),
        max_ver: tuple[int] = (1000,),
        *,
        default: Any = None,
        repeat: int = 1,
        remaining_compressed: bool = False,
        on_read: list[Callable[[RetrieverSub, BaseStructSub], None]] | None = None,
        on_write: list[Callable[[RetrieverSub, BaseStructSub], None]] | None = None,
        mappers: list[Callable[[RetrieverSub, BaseStructSub, T], T]] | None = None,
        validators: list[Callable[[RetrieverSub, BaseStructSub, Any], tuple[bool, str]]] | None = None,
        on_get: list[Callable[[RetrieverSub, BaseStructSub], None]] | None = None,
        on_set: list[Callable[[RetrieverSub, BaseStructSub], None]] | None = None,
    ):
        super().__init__(mappers, validators, on_get, on_set)
        self.dtype = dtype
        self.min_ver = min_ver
        self.max_ver = max_ver
        self.default = default
        self._repeat = repeat
        self.remaining_compressed = remaining_compressed
        self.on_read = on_read or []
        self.on_write = on_write or []

        # if self._repeat == 1:
        #     self.validators.append(lambda retriever, instance, x: dtype.is_valid(x))
        # else:
        #     self.validators.append(lambda retriever, instance, iterable: all(map(dtype.is_valid, iterable)))

    def supported(self, ver: tuple[int, ...]) -> bool:
        return self.min_ver < ver < self.max_ver

    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None:
        super().__set_name__(owner, name)
        owner.add_retriever(self)

    def __set__(self, instance: BaseStruct, value: Any) -> None:
        if not self.supported(instance.struct_version):
            raise VersionError(
                f"{self.p_name!r} is not supported in your scenario version {ver_str(instance.struct_version)!r}"
            )

        if isinstance(value, BaseStruct):
            value.parent = instance

        super().__set__(instance, value)

    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]):
        if instance is None:
            return self

        if not self.supported(instance.struct_version):
            raise VersionError(
                f"{self.p_name!r} is not supported in your scenario version {ver_str(instance.struct_version)!r}"
            )
        try:
            return super().__get__(instance, owner)
        except AttributeError:
            if self.default is None:
                raise ValueError(f"No default value specified for retriever {self.p_name!r}")

            if self.repeat(instance) == 1:
                super().__set__(instance, self.default)
                return self.default

            ls = [self.default] * self.repeat(instance)
            super().__set__(instance, ls)
            return ls

    @property
    def r_name(self) -> str:
        return f"_repeat_{self.p_name}"

    def set_repeat(self, instance: BaseStruct, repeat: int) -> None:
        setattr(instance, self.r_name, repeat)

    def repeat(self, instance: BaseStruct) -> int:
        if (repeat := getattr(instance, self.r_name, None)) is not None:
            return repeat
        return self._repeat

    def from_stream(self, instance: BaseStruct, stream: ByteStream):
        if not self.supported(instance.struct_version):
            return

        repeat = self.repeat(instance)
        if repeat == -1:
            setattr(instance, self.p_name, None)
            return

        def getobj():
            if type(self.dtype) == ABCMeta and issubclass(self.dtype, BaseStruct):
                return self.dtype.from_stream(stream, struct_version = instance.struct_version, parent = instance)
            return self.dtype.from_stream(stream, struct_version = instance.struct_version)

        if repeat == 1 and not hasattr(instance, self.r_name):
            setattr(instance, self.p_name, getobj())
            return

        ls: list = [None] * repeat
        for i in range(repeat):
            ls[i] = getobj()
        setattr(instance, self.p_name, ls)

        for func in self.on_read:
            func(self, instance)

    def to_bytes(self, instance: BaseStruct) -> bytes:
        if not self.supported(instance.struct_version):
            return b""

        repeat = self.repeat(instance)
        if repeat == -1:
            return b""

        for func in self.on_write:
            func(self, instance)

        if repeat == 1 and not hasattr(instance, self.r_name):
            return self.dtype.to_bytes(getattr(instance, self.p_name))

        ls: list = getattr(instance, self.p_name)
        if not len(ls) == repeat:
            raise ValueError(f"length of {self.p_name!r} is not the same as {repeat = }")

        bytes_: list[bytes] = [b""] * repeat

        for j, value in enumerate(getattr(instance, self.p_name)):
            bytes_[j] = self.dtype.to_bytes(value)

        return b"".join(bytes_)
