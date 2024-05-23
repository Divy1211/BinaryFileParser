from testing.sections.scenario_sections import ScenarioSections

def main():
    # todo: correctly initialise struct_ver `from_default` for all self versioned structs
    # todo: for default values that are different across different versions, use default_factory
    # scx = ScenarioSections.from_file(r"C:\Users\Divy\My Stuff\Roll_The_Dice_v9-2_DE.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_36.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_37.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_40.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_41.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_42.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_43.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_44.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_45.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_46.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_47.aoe2scenario")
    # print(scx.data_header.version)

    # scx.to_file(r"C:\Users\Divy\Games\Age of Empires 2 DE\76561198276345085\resources\_common\scenario\test.aoe2scenario")


if __name__ == "__main__":
    main()
