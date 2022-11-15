from src.retrievers.Retriever import Retriever
from src.types.Array import Array32
from src.types.BaseStruct import BaseStruct
from src.types.Bytes import Bytes
from src.types.Int import UInt32
from src.types.Str import NullTermStr32


class Variable(BaseStruct):
    id: int = Retriever(UInt32, default = 0)
    name: str = Retriever(NullTermStr32, default = "_Variable0")


class VariableData(BaseStruct):
    variables: list[Variable] = Retriever(Array32(Variable), default = [])
    unused: bytes = Retriever(Bytes[9], default = b"\x00"*9)
