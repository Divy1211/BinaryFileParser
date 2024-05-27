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

    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/A New Emporer.scn")
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/Age of Heroes b1-3-5.scx")
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/Bronze Age Art of War.scn")
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/CAMELOT.SCN")
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/CEASAR.scn")
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/Corlis.aoescn")
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/Dawn of a New Age.scn")
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/El advenimiento de los hunos_.scx", strict = False)
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/Hotkey Trainer Buildings.aoe2scenario")
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/Jeremiah Johnson (Update).scx")
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/layertest.aoe2scenario")
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/real_world_amazon.scx")
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/The Destruction of Rome.scn")
    # scx = ScenarioSections.from_file(r"./scxs/genie-rs/Year_of_the_Pig.aoe2scenario")

    # print(scx.options.separator)



if __name__ == "__main__":
    main()
