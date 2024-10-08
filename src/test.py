from bfp_rs.types import BfpType
from bfp_rs.types.le import int8
from bfp_rs import Retriever, BaseStruct, ByteStream

from utils import timed


class Test(BaseStruct):
    one = Retriever(BfpType.Int8(int8()), repeat = 60_000_000)

# class Test2(BaseStruct):
#     uwu = Retriever(Struct[Test])

with timed():
    test = Test.test_from_stream(ByteStream.from_file(r"C:\Users\Divy\PycharmProjects\BinaryFileParser\asp_test\test.dat"))


# print(test.uwu)
# # print(Test())
# # print(Test())
# test.uwu = test
#
# print(test.uwu)
