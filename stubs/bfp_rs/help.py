from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from bfp_rs.retrievers import Retriever, RetrieverCombiner, RetrieverRef


def ret(val) -> Retriever:
    """
    Type cast a value as a Retriever. Use this when operating on a retriever object itself i.e. when a property is accessed
    from the class instead of an instance
    """
    return val

def ref(val) -> RetrieverRef:
    """
    Type cast a value as a Reference. Use this when operating on a reference object itself i.e. when a property is accessed
    from the class instead of an instance
    """
    return val

def com(val) -> RetrieverCombiner:
    """
    Type cast a value as a Combiner. Use this when operating on a combiner object itself i.e. when a property is accessed
    from the class instead of an instance
    """
    return val
