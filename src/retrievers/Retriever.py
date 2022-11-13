from __future__ import annotations

from typing import Type, TYPE_CHECKING, Any, Callable, IO

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.retrievers.MapValidate import MapValidate
from src.errors.VersionError import VersionError
from src.types.ParserType import ParserType

if TYPE_CHECKING:
    from src.types.BaseStruct import BaseStruct


def ver_str(ver: tuple[int]) -> str:
    return ".".join(map(str, ver))


class Retriever(MapValidate):
    __slots__ = "cls_or_obj", "min_ver", "max_ver", "default", "_repeat", "remaining_compressed"

    def __init__(
        self,
        cls_or_obj: Type[ParserType] | ParserType,
        min_ver: tuple[int] = (-1,),
        max_ver: tuple[int] = (1000,),
        *,
        default: Any = None,
        remaining_compressed = False,
        repeat: int = 1,
        mappers: list[Callable[[Any], Any]] = None,
        validators: list[Callable[[Any], tuple[bool, str]]] = None,
        on_get: list[Callable[[Retriever, BaseStruct, Type[BaseStruct]], None]] = None,
        on_set: list[Callable[[Retriever, BaseStruct], None]] = None,
    ):
        super().__init__(mappers, validators, on_get, on_set)
        self.cls_or_obj = cls_or_obj
        self.min_ver = min_ver
        self.max_ver = max_ver
        self.default = default
        self._repeat = repeat
        self.remaining_compressed = remaining_compressed

        # if class/object overrides the is_valid method
        if 'is_valid' in cls_or_obj.__dict__:
            self.validators.append(cls_or_obj.is_valid)

    def supported(self, ver: tuple[int]) -> bool:
        return self.min_ver < ver < self.max_ver

    def __set_name__(self, owner: Type[BaseStruct], name: str) -> None:
        super().__set_name__(owner, name)
        owner.add_retriever(self)

    def __set__(self, instance: BaseStruct, value: Any) -> None:
        if not self.supported(instance.file_version):
            raise VersionError(
                f"{self.p_name!r} is not supported in your scenario version {ver_str(instance.file_version)!r}"
            )
        super().__set__(instance, value)

    def __get__(self, instance: BaseStruct, owner: Type[BaseStruct]):
        if instance is None:
            return self

        if not self.supported(instance.file_version):
            raise VersionError(
                f"{self.p_name!r} is not supported in your scenario version {ver_str(instance.file_version)!r}"
            )
        try:
            return super().__get__(instance, owner)
        except AttributeError:
            if self.default is None:
                raise ValueError(f"No default value specified for retriever {self.p_name!r}")

            if self.repeat(instance) == 1:
                super().__set__(instance, self.default)
                return self.default

            ls = [self.default]*self.repeat(instance)
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

    def from_generator(self, instance: BaseStruct, igen: IncrementalGenerator):
        if not self.supported(instance.file_version):
            return

        if self.repeat(instance) == 0:
            setattr(instance, self.p_name, None)
            return

        if self.repeat(instance) == 1:
            setattr(instance, self.p_name, self.cls_or_obj.from_generator(igen, file_version = instance.file_version))
            return

        ls: list = [None] * self.repeat(instance)
        for i in range(self.repeat(instance)):
            ls[i] = self.cls_or_obj.from_generator(igen, file_version = instance.file_version)
        setattr(instance, self.p_name, ls)

    def to_bytes(self, instance: BaseStruct) -> bytes:
        if not self.supported(instance.file_version):
            return b""

        if self.repeat(instance) == 0:
            return b""

        if self.repeat(instance) == 1:
            return self.cls_or_obj.to_bytes(getattr(instance, self.p_name))

        ls: list = getattr(instance, self.p_name)
        if not len(ls) == self.repeat(instance):
            raise ValueError(f"length of {self.p_name!r} is not the same as {self.repeat(instance) = }")

        ls: list[bytes] = [b""] * self.repeat(instance)
        for j, value in enumerate(getattr(instance, self.p_name)):
            ls[j] = self.cls_or_obj.to_bytes(value)

        return b"".join(ls)
