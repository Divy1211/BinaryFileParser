from testing.sections.ScenarioSections import ScenarioSections


def main():
    # for ver in [f"1_{v}" for v in range(36, 48)]:
    #     if ver in ["1_38", "1_39"]:
    #         continue
    # ver = "1_47"
    scx = ScenarioSections.from_file(r"C:\Users\LENOVO PC\Games\Age of Empires 2 DE\76561198276345085\resources\_common\scenario\dd.aoe2scenario")
    # scx.to_file(rf"C:\Users\LENOVO PC\Games\Age of Empires 2 DE\76561198276345085\resources\_common\scenario\ff.aoe2scenario")


if __name__ == "__main__":
    main()
