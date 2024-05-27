from testing.sections.scenario_sections import ScenarioSections


def main():
    # todo: correctly initialise struct_ver `from_default` for all self versioned structs
    # todo: for default values that are different across different versions, use default_factory
    scx = ScenarioSections.from_file(r"./scxs/1_21.scx")
    scx = ScenarioSections.from_file(r"./scxs/1_21.aoe2scenario")
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
    scx = ScenarioSections.from_file(r"./scxs/1_48.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_49.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_51.aoe2scenario")
    scx = ScenarioSections.from_file(r"./scxs/1_53.aoe2scenario")


if __name__ == "__main__":
    main()
