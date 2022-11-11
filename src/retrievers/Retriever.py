from __future__ import annotations

from functools import partial
from typing import Type, TYPE_CHECKING, Any, Callable

from src.retrievers.MapValidate import MapValidate
from src.errors.VersionError import VersionError
from src.types.ParserType import ParserType
from src.types.Str import chk_len

if TYPE_CHECKING:
    from src.sections.BaseSection import BaseSection


def ver_str(ver: tuple[int]) -> str:
    return ".".join(map(str, ver))


class Retriever(MapValidate):
    __slots__ = "min_ver", "cls", "max_ver", "default", "repeat"

    def __init__(
        self,
        cls: Type[ParserType] | ParserType,
        min_ver: tuple[int] = (-1,),
        max_ver: tuple[int] = (1000,),
        *,
        default: Any = None,
        repeat: int = 1,
        mappers: list[Callable[[Any], Any]] = None,
        validators: list[Callable[[Any], tuple[bool, str]]] = None
    ):
        super().__init__(mappers, validators)
        self.cls = cls
        self.min_ver = min_ver
        self.max_ver = max_ver
        self.default = default
        self.repeat = repeat

        if self.repeat > 1:
            self.validators.append(partial(chk_len, self.repeat))

    def supported(self, ver: tuple[int]) -> bool:
        return self.min_ver < ver < self.max_ver

    def __set_name__(self, owner: Type[BaseSection], name: str) -> None:
        super().__set_name__(owner, name)
        owner.add_retriever(self)

    def __set__(self, instance: BaseSection, value: Any) -> None:
        if not self.supported(instance.file_version):
            raise VersionError(
                f"{self.p_name!r} is not supported in your scenario version {ver_str(instance.file_version)!r}"
            )
        super().__set__(instance, value)

    def __get__(self, instance: BaseSection, owner: Type[BaseSection]):
        if not self.supported(instance.file_version):
            raise VersionError(
                f"{self.p_name!r} is not supported in your scenario version {ver_str(instance.file_version)!r}"
            )
        try:
            return super().__get__(instance, owner)
        except AttributeError:
            if self.repeat == 1:
                super().__set__(instance, self.default)
                return self.default

            ls = [self.default]*self.repeat
            super().__set__(instance, ls)
            return ls
