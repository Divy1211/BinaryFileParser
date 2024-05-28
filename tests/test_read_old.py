from tests.sections import ScenarioSections


def test_1_21a():
    ScenarioSections.from_file(r"scxs/1_22.scx")

def test_1_21b():
    ScenarioSections.from_file(r"scxs/1_26.aoe2scenario")

def test_1_11a():
    ScenarioSections.from_file(r"./scxs/genie-rs/A New Emporer.scn")

def test_1_21c():
    ScenarioSections.from_file(r"./scxs/genie-rs/Age of Heroes b1-3-5.scx")

def test_1_10():
    ScenarioSections.from_file(r"./scxs/genie-rs/Bronze Age Art of War.scn")

def test_1_18():
    ScenarioSections.from_file(r"./scxs/genie-rs/CAMELOT.SCN")

def test_1_11b():
    ScenarioSections.from_file(r"./scxs/genie-rs/CEASAR.scn")

def test_1_07a():
    ScenarioSections.from_file(r"./scxs/genie-rs/Dawn of a New Age.scn")

def test_1_11c():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/El advenimiento de los hunos_.scx")

def test_1_36():
    ScenarioSections.from_file(r"./scxs/genie-rs/Hotkey Trainer Buildings.aoe2scenario")

def test_1_11d():
    ScenarioSections.from_file(r"./scxs/genie-rs/Jeremiah Johnson (Update).scx")

def test_1_37():
    ScenarioSections.from_file(r"./scxs/genie-rs/layertest.aoe2scenario")

def test_1_21():
    ScenarioSections.from_file(r"./scxs/genie-rs/real_world_amazon.scx")

def test_1_11e():
    ScenarioSections.from_file(r"./scxs/genie-rs/The Destruction of Rome.scn")

def test1_1_21():
    ScenarioSections.from_file(r"./scxs/genie-rs/Year_of_the_Pig.aoe2scenario")
