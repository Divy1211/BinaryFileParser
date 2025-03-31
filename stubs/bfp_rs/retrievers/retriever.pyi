from __future__ import annotations
from typing import Any, Callable

from bfp_rs.types import Version, BaseStruct

from bfp_rs.combinators.combinator import Combinator


class Retriever:
    """
    Used inside a ``BaseStruct`` subclass to define its serialization schema from built-in types or other base structs,
    allowing setting constraints and hooks
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
        Create a new retriever property to be used for serialization inside a ``BaseStruct`` subclass

        Args:
            data_type: The type of the property used for serialization

            min_ver:
                The minimum struct version which supports this retriever property. If the version of the struct being
                serialized is less than ``min_ver``, serialization for this retriever property is skipped.
                A ``VersionError`` will be raised if an attempt to access this property is made in an un-supporting
                struct. Using SemVer is recommended: https://semver.org/

            max_ver:
                The maximum struct version which supports this retriever property. If the version of the struct being
                serialized is greater than ``max_ver``, serialization for this retriever property is skipped.
                A ``VersionError`` will be raised if an attempt to access this property is made in an un-supporting
                struct. Using SemVer is recommended: https://semver.org/

            default_factory: A function that will receive a version when called and must return an instance of data_type

            repeat:
                The number of times this value is repeated. Prefer using fixed length array types over repeats wherever
                possible. Allowed values for this parameter are:

                - ``-2``: skip a list, sets the property to ``None``
                - ``-1``: skip a value, sets the property to ``None``
                - `` 0``: skip a list, set property to []
                - `` 1``: serialize a value
                - ``>1``: serialize a list

                Note: ``-2`` and ``-1`` both skip serialization but ``-2`` indicates that the true type of the value of
                this retriever property is actually a ``list[data_type]`` (compared to just ``data_type`` when it's
                ``-1``) - this information is needed for type checking if the ``None`` value is later set to an actual
                value

            remaining_compressed:
                If set to ``True``, the ``_decompress``/``_compress`` methods from the ``BaseStruct`` subclass are used
                on the remaining bytes before deserialization/serialization of the remaining properties occurs
                (inclusive)

            on_read:
                A function that must return a list of ``Combinator``s to use for fine-grained operations during
                deserialization

            on_write:
                A function that must return a list of ``Combinator``s to use for fine-grained operations during
                serialization
        """



    def supported(self, ver: Version) -> bool:
        """
        Checks if this property is supported in the given version. A property is supported if
        ``min_ver <= ver <= max_ver``

        Args:
            ver: the version to check support for
        """


    def __get__(self, instance: Any, owner: Any) -> Any: ...

    def __set__(self, instance: BaseStruct, value: Any) -> None: ...

    def __set_name__(self, owner: Any, name: str) -> None: ...
