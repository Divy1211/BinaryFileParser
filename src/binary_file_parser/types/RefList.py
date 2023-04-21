from __future__ import annotations

from typing import Generic, Iterable, overload, TYPE_CHECKING, TypeVar, SupportsIndex

from binary_file_parser.utils import Version

if TYPE_CHECKING:
    from binary_file_parser.retrievers import BaseStruct

T = TypeVar("T")


class RefList(list, Generic[T]):
    def __init__(self, iterable: Iterable[T], struct_ver = Version((0,)), parent: BaseStruct = None):
        # todo: set idxs on structs where relevant
        super().__init__(iterable)
        self.struct_ver = struct_ver
        self.parent = parent

    def set_sub_attrs(self, obj: T):
        if getattr(obj, 'is_struct', False) or isinstance(obj, RefList):
            obj.parent = self.parent
            obj.struct_ver = self.struct_ver

    @property
    def struct_ver(self) -> Version:
        return self._struct_ver

    @struct_ver.setter
    def struct_ver(self, new_struct_ver: Version):
        self._struct_ver = new_struct_ver
        for obj in self:
            if getattr(obj, 'is_struct', False) or isinstance(obj, RefList):
                obj.struct_ver = self.struct_ver

    @property
    def parent(self) -> BaseStruct:
        return self._parent

    @parent.setter
    def parent(self, new_parent: BaseStruct):
        self._parent = new_parent
        for obj in self:
            if getattr(obj, 'is_struct', False) or isinstance(obj, RefList):
                obj.parent = self.parent

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
        return RefList(super().__add__(x), self.struct_ver, self.parent)

    def __iadd__(self: RefList[T], x: Iterable[T]) -> RefList[T]:
        return RefList(super().__iadd__(x), self.struct_ver, self.parent)

    def __radd__(self, other: RefList[T]) -> RefList[T]:
        return self.__add__(other)
