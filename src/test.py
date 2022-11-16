from src.sections.ScenarioSections import ScenarioSections


def main():
    scx = ScenarioSections.from_file("ld.aoe2scenario")
    scx.to_file("ld_wr.aoe2scenario")

if __name__ == "__main__":
    main()
