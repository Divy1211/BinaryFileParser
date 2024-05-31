from binary_file_parser import BaseStruct


class Manager:
    """
    Superclass for creating grouped retriever references and functions on them. Use this to provide a more coherent API
    for struct modification when the internal struct is messy
    """
    __slots__ = '_struct'

    def __init__(self, struct: BaseStruct):
        self._struct = struct
