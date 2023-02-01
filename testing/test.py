from testing.sections.ScenarioSections import ScenarioSections


def main():
    # for ver in [f"1_{v}" for v in range(36, 48)]:
    #     if ver in ["1_38", "1_39"]:
    #         continue
    ver = "1_47"
    scx = ScenarioSections.from_file(rf"{ver}.aoe2scenario")
    with open("./test.txt", "w") as file:
        file.write(f"{scx!r}")
    # print(*map(lambda x: (x.initial_camera_x, x.initial_camera_y, x.editor_camera_x, x.editor_camera_y), scx.unit_data.player_data3), sep = "\n")
    # scx.to_file(rf"first_{ver}.aoe2scenario")


if __name__ == "__main__":
    main()
