from bfp_rs.types import BfpType
from bfp_rs.types.le import int8
from bfp_rs import Retriever, BaseStruct

from utils import timed


class Test(BaseStruct):
    ls = Retriever(BfpType.Int8(int8()), repeat = 60_000_000)

with timed():
    test = Test.from_file(r"C:\Users\Divy\PycharmProjects\BinaryFileParser\asp_test\test.dat")

print(test.ls[24])
