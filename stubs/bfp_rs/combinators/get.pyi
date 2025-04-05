from bfp_rs.retrievers import Retriever


class Get:
    """
    Encodes and performs manipulations on retriever values before passing them to combinators as inputs. For example, it
    can be used to add the value of two retrievers.
    """

    def __add__(self, other: Get | int) -> Get: ...
    def __radd__(self, other: Get | int) -> Get: ...
    def __sub__(self, other: Get | int) -> Get: ...
    def __rsub__(self, other: Get | int) -> Get: ...
    def __mul__(self, other: Get | int) -> Get: ...
    def __rmul__(self, other: Get | int) -> Get: ...
    def __div__(self, other: Get | int) -> Get: ...
    def __rdiv__(self, other: Get | int) -> Get: ...
    def __mod__(self, other: Get | int) -> Get: ...
    def __rmod__(self, other: Get | int) -> Get: ...
    def __and__(self, other: Get | int) -> Get: ...
    def __rand__(self, other: Get | int) -> Get: ...
    def __or__(self, other: Get | int) -> Get: ...
    def __ror__(self, other: Get | int) -> Get: ...
    def __xor__(self, other: Get | int) -> Get: ...
    def __rxor__(self, other: Get | int) -> Get: ...
    def __neg__(self) -> Get: ...
    def __invert__(self) -> Get: ...



def get(*source: Retriever | int) -> Get:
    """
    Fetches the value of a retriever. If the resulting value is an ``int``, it's result can be manipulated with ``int``
    operations. For example, ``get(Struct1.sub_struct1, SubStruct1.sub_list1, 1, SubStruct2.property)+1`` is like doing
    ``struct1.sub_struct1.sub_list1[1].property+1``

    Args:
        *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
            starting in the current struct

    Returns:
        An instance encoding the retriever path and manipulations to perform during parsing

    Raises:
        ValueError: If the value at the source path is not an ``int``, and ``int`` operations are attempted
    """


def get_len(*source: Retriever | int) -> Get:
    """
    Fetches the length of a list-valued retriever. It's result can be manipulated with operations. For example,
    ``get_len(Struct1.sub_struct1, SubStruct1.sub_list1)-1`` is like doing ``len(struct1.sub_struct1.sub_list1)-1``

    Args:
        *source: The retriever path to fetch the value from. This can be a sequence of retrievers/list indices
            starting in the current struct

    Returns:
        An instance encoding the retriever path and manipulations to perform during parsing

    Raises:
        ValueError: If the value at the source path is not a list
    """

