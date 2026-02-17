import sys

# sys.path.insert(0, r"C:/Users/Divy/PycharmProjects/bfp-rs")
# sys.path.insert(0, r"/mnt/c/Users/Divy/PycharmProjects/bfp-rs/")

from asp_test.sections import ScenarioSections
from asp_test.utils import timed

with timed("read"):
    scx = ScenarioSections.from_file(r"C:/Users/dchandra/Games/Age of Empires 2 DE/76561199834870459/resources/_common/scenario/Path_Blood - Copy 2.aoe2scenario", strict = False)
    # scx = ScenarioSections.from_file(r"/mnt/c/Users/Divy/PycharmProjects/BinaryFileParser/asp_test/scxs/Bomberman 8p_1_47.aoe2scenario")
    print(scx.trigger_data.ver)
    print(len(scx.trigger_data.triggers[0].effects[0]._properties))
    print(len(scx.trigger_data.triggers[1].conditions[0]._properties))

# with timed("write"):
#     scx.to_json("./jtest.json")
    # scx.to_file("./wtest.bin")

# with timed("read_json"):
#     scx2 = ScenarioSections.from_json("./jtest.json")
#
# print(scx.trigger_data.ver)
