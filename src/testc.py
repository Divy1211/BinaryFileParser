from bfp_rs.types import Version
from bfp_rs.types.le import i8
from bfp_rs.combinators import set_repeat, if_, if_not, if_len
from bfp_rs import Retriever, BaseStruct, ByteStream, Struct

from utils import timed


class Test2(BaseStruct):
    num = Retriever(i8)


class Test(BaseStruct):
    one = Retriever(i8)

class Test2(BaseStruct):
    uwu = Retriever(Struct[Test], repeat = 60_000_000)

with timed():
    test = Test2.from_file(r"C:\Users\Divy\PycharmProjects\BinaryFileParser\asp_test\test.dat")
