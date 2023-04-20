from __future__ import annotations

from typing import Generic, Iterable, overload, TypeVar, SupportsIndex

from binary_file_parser.retrievers import BaseStruct
from binary_file_parser.utils import Version

T = TypeVar("T")


class RefList(list, Generic[T]):
    def __init__(self, iterable: Iterable[T], parent: BaseStruct = None, struct_ver = Version((0,))):
        super().__init__(iterable)
        self.parent = parent
        self.struct_ver = struct_ver

    def set_sub_attrs(self, obj: T):
        if isinstance(obj, (BaseStruct, RefList)):
            obj.parent = self.parent
            obj.struct_ver = self.struct_ver

    @property
    def struct_ver(self) -> Version:
        return self._struct_ver

    @struct_ver.setter
    def struct_ver(self, new_struct_ver: Version):
        self._struct_ver = new_struct_ver
        for obj in self:
            self.set_sub_attrs(obj)

    @property
    def parent(self) -> BaseStruct:
        return self._parent

    @parent.setter
    def parent(self, new_parent: BaseStruct):
        self._parent = new_parent
        for obj in self:
            self.set_sub_attrs(obj)

    def append(self, obj: T) -> None:
        super().append(obj)
        self.set_sub_attrs(obj)

    def extend(self, iterable: Iterable[T]) -> None:
        super().extend(iterable)
        for obj in iterable:
            self.set_sub_attrs(obj)

    def insert(self, index: SupportsIndex, obj: T) -> None:
        super().insert(index, obj)
        self.set_sub_attrs(obj)

    @overload
    def __setitem__(self, i: SupportsIndex, o: T) -> None: ...

    @overload
    def __setitem__(self, s: slice, o: Iterable[T]) -> None: ...

    def __setitem__(self, i: SupportsIndex | slice, o: T | Iterable[T]) -> None:
        self.set_sub_attrs(o)
        super().__setitem__(i, o)

    def __add__(self, x: RefList[T]) -> RefList[T]:
        for obj in x:
            self.set_sub_attrs(obj)
        return RefList(super().__add__(x))

    def __iadd__(self: RefList[T], x: Iterable[T]) -> RefList[T]:
        for obj in x:
            self.set_sub_attrs(obj)
        return RefList(super().__iadd__(x))
