from bfp_rs.types import BfpType, Version
from bfp_rs.types.le import int8
from bfp_rs import Retriever, BaseStruct, ByteStream, Struct

from utils import timed


class SubTest(BaseStruct):
    num: int = Retriever(BfpType.Int8(int8()))


class Test(BaseStruct):
    ls = Retriever(Struct[SubTest], repeat = 60_000_000, remaining_compressed = True)

    @classmethod
    def _decompress(cls, bytes_: bytes) -> bytes:
        return bytes_


with timed():
    test = Test.from_file(r"C:\Users\Divy\PycharmProjects\BinaryFileParser\asp_test\test.dat")

print(test.ver)

print(test.ls[24])
