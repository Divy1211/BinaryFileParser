from __future__ import annotations
from typing import Type, TYPE_CHECKING, Any, Callable

from src.retrievers.MapValidate import MapValidate
from src.errors.VersionError import VersionError
from src.types.ParserType import ParserType

if TYPE_CHECKING:
    from src.sections.BaseSection import BaseSection

class Retriever(MapValidate):
    __slots__ = "min_ver", "cls", "max_ver", "default", "ver"

    def __init__(
        self,
        cls: Type[ParserType] | ParserType,
        min_ver: tuple[int, int] = (-2, -100),
        max_ver: tuple[int, int] = (2, 100),
        *,
        default: Any = None,
        mappers: list[Callable[[Any], Any]] = None,
        validators: list[Callable[[Any], tuple[bool, str]]] = None
    ):
        super().__init__(mappers, validators)
        self.cls = cls
        self.min_ver = min_ver
        self.max_ver = max_ver
        self.default = default
        self.ver = (-1, -1)

    @property
    def supported(self) -> bool:
        return self.min_ver < self.ver < self.max_ver

    def __set_name__(self, owner: Type[BaseSection], name: str) -> None:
        super().__set_name__(owner, name)
        owner.add_retriever(self)

    def __set__(self, instance: BaseSection, value: Any) -> None:
        if not self.supported:
            raise VersionError(f"This attribute is not supported in your scenario version ({self.ver})")
        super().__set__(instance, value)

    def __get__(self, instance: BaseSection, owner: Type[BaseSection]):
        if not self.supported:
            raise VersionError(f"This attribute is not supported in your scenario version ({self.ver})")
        try:
            return super().__get__(instance, owner)
        except AttributeError:
            super().__set__(instance, self.default)
            return self.default
