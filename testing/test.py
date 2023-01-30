from testing.sections.ScenarioSections import ScenarioSections


def main():
    scx = ScenarioSections.from_file("1_46.aoe2scenario")
    print(scx.trigger_data.num_triggers)
    scx.to_file("first_1_46.aoe2scenario")

if __name__ == "__main__":
    main()
