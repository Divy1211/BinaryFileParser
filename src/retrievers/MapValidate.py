from __future__ import annotations

from typing import Callable, Type, Any, TypeVar

T = TypeVar("T")


class MapValidate:
    __slots__ = "p_name", "s_name", "mappers", "validators", "on_get", "on_set"

    def __init__(
        self,
        mappers: list[Callable[[MapValidate, Any, T], T]] = None,
        validators: list[Callable[[MapValidate, Any, Any], tuple[bool, str]]] = None,
        on_get: list[Callable[[MapValidate, Any, Type], None]] = None,
        on_set: list[Callable[[MapValidate, Any], None]] = None,
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
            func(self, instance, owner)

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
