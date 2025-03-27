from __future__ import annotations
from typing import Any, Callable

from bfp_rs.types import Version, BaseStruct

from bfp_rs.combinators.combinator import Combinator


class Retriever:
    """
    Defines a struct's parsing schema from built-in types, allowing setting constraints and hooks
    """

    def __new__(
        cls,
        data_type: Any,
        *,
        min_ver: Version = Version(-1),
        max_ver: Version = Version(10_000),
        default: Any = None,
        default_factory: Callable[[Version], Any] = None,
        repeat: int = 1,
        remaining_compressed: bool = False,
        on_read: Callable[[], list[Combinator]] = None,
        on_write: Callable[[], list[Combinator]] = None,
    ) -> Retriever:
        """
        Defines a struct's parsing schema from built-in types, allowing setting constraints and hooks

        Args:
            data_type: The type of value to read
            min_ver:
                The minimum struct version which supports this retriever property. If the version of the struct being
                read is less than min_ver, reading this retriever property is skipped and a version error is raised if
                an attempt to access or assign it is made. Using SemVer is recommended: https://semver.org/
            max_ver:
                The maximum struct version which supports this retriever property. If the version of the struct being
                read is greater than max_ver, reading this retriever property is skipped and a version error is raised
                if an attempt to access or assign it is made. Using SemVer is recommended: https://semver.org/
            default: A default value for this retriever property. Only use this for primitives

            default_factory: A function that will receive a version when called and must return an instance of data_type
            repeat:
                The number of times this value is repeated. Possible values for this parameter include:
                    ``-2``: skip a list, sets the property to None
                    ``-1``: skip a value, sets the property to None
                    `` 0``: skip a list, set property to []
                    `` 1``: read a value
                    ``>1``: read a list
            remaining_compressed:
                If set to true, the _decompress/_compress methods are used on the remaining bytes before
                reading/writing the remaining retriever properties
            on_read:
                A function that must return a list of ``Combinator``s to use for fine-grained operations during reading
            on_write:
                A function that must return a list of ``Combinator``s to use for fine-grained operations during writing
        """
        ...


    def supported(self, ver: Version) -> bool:
        """
        Checks if this property is supported in the given version. A property is supported if min_ver <= ver <= max_ver

        Args:
            ver: the version to check support for
        """
        ...

    def __get__(self, instance: Any, owner: Any) -> Any: ...

    def __set__(self, instance: BaseStruct, value: Any) -> None: ...

    def __set_name__(self, owner: Any, name: str) -> None: ...
