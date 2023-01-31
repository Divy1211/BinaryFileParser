from testing.sections.ScenarioSections import ScenarioSections


def main():
    ver = "1_43"
    scx = ScenarioSections.from_file(f"{ver}.aoe2scenario")
    scx.to_file(f"first_{ver}.aoe2scenario")


if __name__ == "__main__":
    main()
