from tests.sections import ScenarioSections


def main():
    scx = ScenarioSections.from_file(r"./scxs/genie-rs/layertest.aoe2scenario")
    print(scx.tribe_scenario.background_image.size)

if __name__ == "__main__":
    main()
