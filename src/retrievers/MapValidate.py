from __future__ import annotations

from typing import Callable, Type, TypeVar

T = TypeVar("T")
Instance = TypeVar("Instance")
MapValidateSub = TypeVar("MapValidateSub", bound = "MapValidate")


class MapValidate:
    """
    A data descriptor that will apply the provided mapper functions to its data to transform it and then validate it
    using the provided validator functions before completing assignment. If any of the validators return false, a
    ValueError is raised.
    The on_get and on_set functions are invoked when the data associated with the descriptor is accessed and set
    respectively
    """
    __slots__ = "p_name", "s_name", "mappers", "validators", "on_get", "on_set"

    def __init__(
        self,
        mappers: list[Callable[[MapValidateSub, Instance, T], T]] | None = None,
        validators: list[Callable[[MapValidateSub, Instance, T], tuple[bool, str]]] | None = None,
        on_get: list[Callable[[MapValidateSub, Instance], None]] | None = None,
        on_set: list[Callable[[MapValidateSub, Instance], None]] | None = None,
    ):
        """
        :param mappers:
            A list of functions that transform the assigned data value in sequence. These will be called with the
            descriptor, the data instance and the data value that was assigned to the descriptor property and must
            return a value of the same type. The output of one mapper function becomes the input of the next.
        :param validators:
            A list of functions that validate the assigned data value in sequence. These will be called with the
            descriptor, the data instance and the data value that was assigned to the descriptor property and must
            return a tuple of boolean and string values. The boolean indicates validity, and when False, a ValueError is
            raised using the returned string as the error message. This string needs to include a format specifier '%s'
            which will be replaced with the name of the descriptor property
        :param on_get:
            A list of functions that will be called when a data value is accessed. These will be called with the
            descriptor and the data instance
        :param on_set:
            A list of functions that will be called when a data value is assigned. These will be called with the
            descriptor and the data instance
        """
        self.mappers = mappers or []
        self.validators = validators or []
        self.on_get = on_get or []
        self.on_set = on_set or []

    def __set_name__(self, owner: Type[Instance], name: str) -> None:
        self.p_name = name
        self.s_name = f"_{name}"

    def __get__(self, instance: Instance, owner: Type[Instance]) -> MapValidate | T:
        if instance is None:
            return self

        for func in self.on_get:
            func(self, instance)

        return getattr(instance, self.s_name)

    def __set__(self, instance: Instance, value: T) -> None:
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
