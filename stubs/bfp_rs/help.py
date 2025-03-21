from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from bfp_rs.retrievers import Retriever, RetrieverCombiner, RetrieverRef


def ret(val) -> Retriever:
    return val

def ref(val) -> RetrieverRef:
    return val

def com(val) -> RetrieverCombiner:
    return val
