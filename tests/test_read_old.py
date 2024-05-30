from tests.sections import ScenarioSections


def test_1_21a():
    scx = ScenarioSections.from_file(r"scxs/1_22.scx")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_21b():
    scx = ScenarioSections.from_file(r"scxs/1_26.aoe2scenario")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_11a():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/A New Emporer.scn")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_21c():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/Age of Heroes b1-3-5.scx")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_10():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/Bronze Age Art of War.scn")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_18():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/CAMELOT.SCN")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_11b():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/CEASAR.scn")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_07a():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/Dawn of a New Age.scn")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_11c():
    scx = scx = ScenarioSections.from_file(r"./scxs/genie-rs/El advenimiento de los hunos_.scx")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_36():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/Hotkey Trainer Buildings.aoe2scenario")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_11d():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/Jeremiah Johnson (Update).scx")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_37():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/layertest.aoe2scenario")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_21():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/real_world_amazon.scx")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_11e():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/The Destruction of Rome.scn")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test1_1_21():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/Year_of_the_Pig.aoe2scenario")
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")
