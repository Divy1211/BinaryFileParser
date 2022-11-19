from src.sections.ScenarioSections import ScenarioSections


def main():
    # scx = ScenarioSections.from_file("1_47.aoe2scenario")
    scx = ScenarioSections()
    scx.to_file("first.aoe2scenario")

if __name__ == "__main__":
    main()
