import os
import sys

from bfp_rs import Version

# drive = "C:"
# drive = "/mnt/c"

# sys.path.insert(0, r"C:/Users/Divy/PycharmProjects/bfp-rs")
# sys.path.insert(0, r"/mnt/c/Users/Divy/PycharmProjects/bfp-rs/")

from asp_test.sections import ScenarioSections
from asp_test.utils import timed

with timed("read"):
    scx = ScenarioSections.from_file(r"C:\Users\dchandra\Games\Age of Empires 2 DE\76561199834870459\resources\_common\scenario\test_155_lc.aoe2scenario")

    # # scx: ScenarioSections = ScenarioSections.from_file(r"C:\Users\Divy\PycharmProjects\BinaryFileParser\asp_test\scxs\Bomberman 8p_1_47.aoe2scenario")
    # # scx: ScenarioSections = ScenarioSections.from_file(r"C:\Users\dchandra\Games\Age of Empires 2 DE\76561199834870459\resources\_common\scenario\food_units.aoe2scenario")
    # # scx: ScenarioSections = ScenarioSections.from_file(r"C:\Users\dchandra\Games\Age of Empires 2 DE\76561199834870459\resources\_common\scenario\test.aoe2scenario", strict = False)
    # # ScenarioSections.to_file(r"C:\Users\dchandra\Games\Age of Empires 2 DE\76561199834870459\resources\_common\scenario\test_w.aoe2scenario", scx)
    #
    # path = r"C:\Users\dchandra\Games\Age of Empires 2 DE\76561199834870459\resources\_common\scenario"
    #
    # for file in os.listdir(path):
    #     name = os.path.join(path, file)
    #
    #     try:
    #         scx: ScenarioSections = ScenarioSections.from_file(name)
    #     except ValueError:
    #         print("error: ", name)
    #         continue
    #
    #     print(file, scx.ver)
    #
    #     if scx.ver != Version(1, 55):
    #         continue
    #
    #     scx.trigger_data.version = 3.9
    #     scx.file_header.file_version = '1.54'
    #
    #     name_out = os.path.join(path, "..", "scenario_154", file)
    #
    #     ScenarioSections.to_file(name_out, scx)
    #     print(name_out)
