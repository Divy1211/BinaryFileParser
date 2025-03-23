from bfp_rs.types import BaseStruct


class Manager:
    """
    Superclass for creating grouped retriever references and functions on them. Use this to provide a more coherent API
    for struct modification when the internal struct is messy
    """
    struct_: BaseStruct

    def __init__(self, struct: BaseStruct): ...
