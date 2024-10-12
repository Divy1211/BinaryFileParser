from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from binary_file_parser import Retriever, RetrieverRef, RetrieverCombiner

def ret(t) -> Retriever:
    """Cast a Retriever to the right type when the type checker doesn't understand class level access"""
    return t

def ref(t) -> RetrieverRef:
    """Cast a RetrieverRef to the right type when the type checker doesn't understand class level access"""
    return t

def com(t) -> RetrieverCombiner:
    """Cast a RetrieverCombiner to the right type when the type checker doesn't understand class level access"""
    return t
