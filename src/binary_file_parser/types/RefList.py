from __future__ import annotations

import copy
from typing import Callable, Generic, Iterable, overload, Type, TYPE_CHECKING, TypeVar, SupportsIndex

from binary_file_parser.utils import Version

if TYPE_CHECKING:
    from binary_file_parser.retrievers import BaseStruct

T = TypeVar("T")


def ref_listify(cls: Type[RefList], struct_ver: Version, parent: BaseStruct) -> Callable[[T], T]:
    def listify(i):
        if not isinstance(i, list):
            return i
        return cls(i, struct_ver, parent)

    return listify


class RefList(list, Generic[T]):
    def __init__(self, iterable: Iterable[T], struct_ver = Version((0,)), parent: BaseStruct = None):
        # todo: set idxs on structs where relevant
        # todo: set own idx for nested lists
        # todo: stop recursive set struct vers and parents if same
        iterable = map(ref_listify(self.__class__, struct_ver, parent), iterable)
        super().__init__(iterable)
        self._struct_ver = struct_ver
        self._parent = parent

    def __deepcopy__(self, memo):
        cls = self.__class__

        cloned_list = copy.deepcopy(list(self), memo)

        reflist = cls(cloned_list, copy.deepcopy(self.struct_ver), None)
        memo[id(self)] = reflist

        return reflist

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
