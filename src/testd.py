from bfp_rs import ret, BaseStruct, Manager, Retriever, RetrieverRef, Context
from bfp_rs.types.le import u8


class Mre(BaseStruct):
    tiles: list[int] = Retriever(u8, default = 1, repeat = 10)


class MreManager(Manager):
    _struct: Mre

    val = RetrieverRef(ret(Mre.tiles))


# mre = Mre.from_bytes(b'\x00\x00\x00\x00\x00\x00\x00\x09\x08\x00')
mre = Mre()

print(f"Pre direct update:   {mre.tiles}")
mre.tiles = list(range(10, 0, -1))
print(f"Post direct update:  {mre.tiles}")

print('Initialising manager...')
manager = MreManager(mre)

print(f"Pre manager update:  {manager.val}")
manager.val = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
print(f"Post manager update:  {manager.val}")
