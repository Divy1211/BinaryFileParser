from __future__ import annotations

from typing import Callable, Type, Any, TypeVar

T = TypeVar("T")
MapValidateSub = TypeVar("MapValidateSub", bound = "MapValidate")


class MapValidate:
    __slots__ = "p_name", "s_name", "mappers", "validators", "on_get", "on_set"

    def __init__(
        self,
        mappers: list[Callable[[MapValidateSub, Any, T], T]] | None = None,
        validators: list[Callable[[MapValidateSub, Any, Any], tuple[bool, str]]] | None = None,
        on_get: list[Callable[[MapValidateSub, Any], None]] | None = None,
        on_set: list[Callable[[MapValidateSub, Any], None]] | None = None,
    ):
        self.mappers = mappers or []
        self.validators = validators or []
        self.on_get = on_get or []
        self.on_set = on_set or []

    def __set_name__(self, owner: Type, name: str):
        self.p_name = name
        self.s_name = f"_{name}"

    def __get__(self, instance: Any, owner: Type):

        for func in self.on_get:
            func(self, instance)

        return getattr(instance, self.s_name)

    def __set__(self, instance: Any, value: Any):
        val = value
        for mapper in self.mappers:
            val = mapper(self, instance, val)

        for validator in self.validators:
            valid, msg = validator(self, instance, val)
            if not valid:
                raise ValueError(msg % repr(self.p_name))

        setattr(instance, self.s_name, val)
        for func in self.on_set:
            func(self, instance)
