from testing.sections.scenario_sections import ScenarioSections

def main():
    scx = ScenarioSections.from_file(r"./scxs/1_47.aoe2scenario")
    # print(scx.data_header.unknown_1_46)
    # scx.data_header.unknown_1_46 = b"\x00\x00"+b"\x00"*7
    # scx = ScenarioSections.from_file(r"C:\Users\Divy\My Stuff\Roll_The_Dice_v9-2_DE.aoe2scenario")

    scx.to_file(r"C:\Users\Divy\Games\Age of Empires 2 DE\76561198276345085\resources\_common\scenario\test.aoe2scenario")


if __name__ == "__main__":
    main()
