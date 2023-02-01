from testing.sections.ScenarioSections import ScenarioSections


def main():
    for ver in [f"1_{v}" for v in range(36, 48)]:
        if ver in ["1_38", "1_39"]:
            continue
        scx = ScenarioSections.from_file(f"{ver}.aoe2scenario")
        scx.to_file(f"first_{ver}.aoe2scenario")


if __name__ == "__main__":
    main()
