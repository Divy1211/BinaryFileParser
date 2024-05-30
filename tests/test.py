from tests.sections import ScenarioSections


def main():
    scx = ScenarioSections.from_file(r"./scxs/1_37.aoe2scenario")
    print(scx.tribe_scenario.player_options.legacy_ai_files[5].ai_rules)



if __name__ == "__main__":
    main()
