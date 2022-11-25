from src.retrievers.Retriever import Retriever
from src.types.Array import Array32
from src.retrievers.BaseStruct import BaseStruct
from src.types.Bytes import Bytes
from src.types.Int import uint32
from src.types.Str import nt_str32


class Variable(BaseStruct):
    id: int = Retriever(uint32, default = 0)
    name: str = Retriever(nt_str32, default = "_Variable0")

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True):
        super().__init__(struct_version, parent, initialise_defaults)


class VariableData(BaseStruct):
    variables: list[Variable] = Retriever(Array32[Variable], default = [])
    unused: bytes = Retriever(Bytes[9], default = b"\x00"*9)

    def __init__(self, struct_version: tuple[int, ...] = (1, 47), parent: BaseStruct = None, initialise_defaults = True):
        super().__init__(struct_version, parent, initialise_defaults)
