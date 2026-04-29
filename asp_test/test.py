import sys

from bfp_rs.diff import Basic, Conflict, NestedConflict, Diff, Inserted, Deleted, Changed, NestedDiff

# sys.path.insert(0, r"C:/Users/Divy/PycharmProjects/bfp-rs")
# sys.path.insert(0, r"/mnt/c/Users/Divy/PycharmProjects/bfp-rs/")

from asp_test.sections import ScenarioSections
from asp_test.utils import timed

with timed("read"):
    # scx1 = ScenarioSections.from_file(r"C:\Users\Divy\Games\Age of Empires 2 DE\76561198276345085\resources\_common\scenario\Dodgeball FFA v2.0.0a1.aoe2scenario")
    scx1 = ScenarioSections.from_file(r"./scxs/1_45.aoe2scenario")
    print(scx1.trigger_data.ver)
