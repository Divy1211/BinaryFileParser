from testing.sections.ScenarioSections import ScenarioSections


def main():
    # for ver in [f"1_{v}" for v in range(36, 48)]:
    #     if ver in ["1_38", "1_39"]:
    #         continue
    ver = f"1_21"
    scx = ScenarioSections.from_file(rf"D:\Unturned\steamapps\common\Age2HD\resources\_common\scenario\from_e.aoe2scenario", strict = False)
    print(scx.file_header.unknowns)
    # scx = ScenarioSections.from_file(rf"default.aoe2scenario", strict = False)
    # print(scx.options.editor_camera_x)
    # print(scx.options.editor_camera_y)
    # scx.to_file(f"first_{ver}.aoe2scenario")


if __name__ == "__main__":
    main()
