from bfp_rs.types import Version, BfpType
from bfp_rs.types.le import int8, int16, float64
from bfp_rs import Retriever


# a = Retriever(float64())

print(BfpType.Rect((1, 2), 10, 10).top_left)
