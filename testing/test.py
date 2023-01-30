from testing.sections.ScenarioSections import ScenarioSections


def main():
    scx = ScenarioSections.from_file("1_45.aoe2scenario")
    scx.to_file("first_1_45.aoe2scenario")

if __name__ == "__main__":
    main()
