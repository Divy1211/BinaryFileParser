from __future__ import annotations
from typing import Any, Callable

from bfp_rs import Version, BaseStruct
from bfp_rs.combinators.combinator import Combinator


class Retriever:
    """
    """

    def __new__(
        cls,
        data_type: Any,
        min_ver: Version = Version(-1),
        max_ver: Version = Version(10_000),
        default: Any = None,
        default_factory: Callable[[Version], Any] = None,
        repeat: int = 1,
        remaining_compressed: bool = False,
        on_read: Callable[[], list[Combinator]] = None,
        on_write: Callable[[], list[Combinator]] = None,
    ) -> Retriever:
        ...


    def supported(self, ver: Version) -> bool: ...

    def __get__(self, instance: Any, owner: Any) -> Any: ...

    def __set__(self, instance: BaseStruct, value: Any) -> None: ...

    def __set_name__(self, owner: Any, name: str) -> None: ...
