from binary_file_parser import Retriever, BaseStruct
from binary_file_parser.types import Bytes, uint32, nt_str32, Array32


class Variable(BaseStruct):
    id: int = Retriever(uint32, default = 0)
    name: str = Retriever(nt_str32, default = "_Variable0")

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True):
        super().__init__(struct_version, parent, initialise_defaults)


class VariableData(BaseStruct):
    variables: list[Variable] = Retriever(Array32[Variable], default = [])
    unused: bytes = Retriever(Bytes[9], default = b"\x00"*9, min_ver = (1, 46))

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True):
        super().__init__(struct_version, parent, initialise_defaults)
