import sys

# drive = "C:"
# drive = "/mnt/c"

# sys.path.insert(0, r"C:/Users/Divy/PycharmProjects/bfp-rs")
sys.path.insert(0, r"/mnt/c/Users/Divy/PycharmProjects/bfp-rs/")

from asp_test.sections import ScenarioSections
from asp_test.utils import timed

with timed("read"):
    # scx: ScenarioSections = ScenarioSections.from_file(r"C:\Users\Divy\PycharmProjects\BinaryFileParser\asp_test\scxs\Bomberman 8p_1_47.aoe2scenario")
    scx: ScenarioSections = ScenarioSections.from_file(r"/mnt/c/Users/Divy/PycharmProjects/BinaryFileParser/asp_test/scxs/Bomberman 8p_1_47.aoe2scenario")
