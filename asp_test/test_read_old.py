from sections import ScenarioSections


def test_1_21a():
    scx = ScenarioSections.from_file(r"scxs/1_22.scx")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_21b():
    scx = ScenarioSections.from_file(r"scxs/1_26.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_11a():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/A New Emporer.scn", strict = False)
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_21c():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/Age of Heroes b1-3-5.scx")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_10():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/Bronze Age Art of War.scn")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_18():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/CAMELOT.SCN")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_11b():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/CEASAR.scn")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_07a():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/Dawn of a New Age.scn")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_11c():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/El advenimiento de los hunos_.scx")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_36():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/Hotkey Trainer Buildings.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_11d():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/Jeremiah Johnson (Update).scx")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_37():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/layertest.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_21():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/real_world_amazon.scx")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_11e():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/The Destruction of Rome.scn")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test1_1_21():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/Year_of_the_Pig.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin", scx)
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def main():
    test_1_21a()
    test_1_21b()
    test_1_11a()
    test_1_21c()
    test_1_10()
    test_1_18()
    test_1_11b()
    test_1_07a()
    test_1_11c()
    test_1_36()
    test_1_11d()
    test_1_37()
    test_1_21()
    test_1_11e()
    test1_1_21()

if __name__ == '__main__':
    main()
