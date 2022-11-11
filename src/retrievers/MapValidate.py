from typing import Callable, Type, Any


class MapValidate:
    __slots__ = "p_name", "s_name", "mappers", "validators"

    def __init__(self, mappers: list[Callable[[Any], Any]] = None, validators: list[Callable[[Any], tuple[bool, str]]] = None):
        self.validators = validators or []
        self.mappers = mappers or []

    def __set_name__(self, owner: Type, name: str):
        self.p_name = name
        self.s_name = f"_{name}"

    def __get__(self, instance: Any, owner: Type):
        return getattr(instance, self.s_name)

    def __set__(self, instance: Any, value: Any):
        val = value
        for mapper in self.mappers:
            val = mapper(val)

        for validator in self.validators:
            valid, msg = validator(val)
            if not valid:
                raise ValueError(msg % repr(self.p_name))

        setattr(instance, self.s_name, val)
