from sections import ScenarioSections


def test_1_36():
    scx = ScenarioSections.from_file(r"./scxs/1_36.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_37():
    scx = ScenarioSections.from_file(r"./scxs/1_37.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_40():
    scx = ScenarioSections.from_file(r"./scxs/1_40.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_41():
    scx = ScenarioSections.from_file(r"./scxs/1_41.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_42():
    scx = ScenarioSections.from_file(r"./scxs/1_42.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_43():
    scx = ScenarioSections.from_file(r"./scxs/1_43.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_44():
    scx = ScenarioSections.from_file(r"./scxs/1_44.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_45():
    scx = ScenarioSections.from_file(r"./scxs/1_45.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_46():
    scx = ScenarioSections.from_file(r"./scxs/1_46.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_47():
    scx = ScenarioSections.from_file(r"./scxs/1_47.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_48():
    scx = ScenarioSections.from_file(r"./scxs/1_48.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_49():
    scx = ScenarioSections.from_file(r"./scxs/1_49.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_51():
    scx = ScenarioSections.from_file(r"./scxs/1_51.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def test_1_53():
    scx = ScenarioSections.from_file(r"./scxs/1_53.aoe2scenario")
    print(scx.settings.data_header.file_name)
    scx.to_file(r"./scxs/wtest.bin")
    ScenarioSections.from_file(r"./scxs/wtest.bin")

def main():
    test_1_36()
    test_1_37()
    test_1_40()
    test_1_41()
    test_1_42()
    test_1_43()
    test_1_44()
    test_1_45()
    test_1_46()
    test_1_47()
    test_1_48()
    test_1_49()
    test_1_51()
    test_1_53()

if __name__ == '__main__':
    main()
