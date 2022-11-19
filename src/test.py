from src.sections.ScenarioSections import ScenarioSections


def main():
    scx = ScenarioSections.from_file("Bomberman 8p_1_47.aoe2scenario")
    scx.to_file("first.aoe2scenario")

if __name__ == "__main__":
    main()
