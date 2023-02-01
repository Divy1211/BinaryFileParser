from testing.sections.ScenarioSections import ScenarioSections


def main():
    # for ver in [f"1_{v}" for v in range(40, 48)]:
    ver = "1_40"
    scx = ScenarioSections.from_file(f"{ver}.aoe2scenario")
    scx.to_file(f"first_{ver}.aoe2scenario")


if __name__ == "__main__":
    main()
