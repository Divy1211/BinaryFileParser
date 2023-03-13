from testing.sections.ScenarioSections import ScenarioSections


def main():
    # for ver in [f"1_{v}" for v in range(36, 48)]:
    #     if ver in ["1_38", "1_39"]:
    #         continue
    # ver = "1_47"
    scx = ScenarioSections.from_file(rf"C:\Users\LENOVO PC\Games\Age of Empires 2 DE\76561198276345085\resources\_common\scenario\dd.aoe2scenario")
    print(scx.file_header.required_dats)
    # with open("./test.txt", "w") as file:
    #     file.write(f"{scx!r}")
    # print(*map(lambda x: (x.initial_camera_x, x.initial_camera_y, x.editor_camera_x, x.editor_camera_y), scx.unit_data.player_data3), sep = "\n")
    # scx.to_file(rf"C:\Users\LENOVO PC\Games\Age of Empires 2 DE\76561198276345085\resources\_common\scenario\ff.aoe2scenario")


if __name__ == "__main__":
    main()
