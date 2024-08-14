from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from binary_file_parser.retrievers import Retriever, RetrieverCombiner, RetrieverRef
    from binary_file_parser.types.base_struct import BaseStruct

class Manager:
    """
    Superclass for creating grouped retriever references and functions on them. Use this to provide a more coherent API
    for struct modification when the internal struct is messy
    """
    __slots__ = '_struct'

    _retrievers: list[Retriever] = []
    _refs: list[RetrieverRef] = []
    _combiners: list[RetrieverCombiner] = []

    @classmethod
    def _add_retriever(cls, retriever: Retriever):
        cls._retrievers.append(retriever)

    @classmethod
    def _add_ref(cls, ref: RetrieverRef):
        cls._refs.append(ref)

    @classmethod
    def _add_combiner(cls, combiner: RetrieverCombiner):
        cls._combiners.append(combiner)

    def __init__(self, struct: BaseStruct):
        self._struct = struct

    # todo: pretty print and degug stuff. same as base struct. perhaps create a superclass.
