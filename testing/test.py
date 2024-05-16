from testing.sections.ScenarioSections import ScenarioSections

def main():
    # scx = ScenarioSections.from_file(r"./scxs/1_47.aoe2scenario")
    scx = ScenarioSections.from_file(r"C:\Users\Divy\My Stuff\Roll_The_Dice_v9-2_DE.aoe2scenario")

    scx.to_file(r"C:\Users\Divy\My Stuff\Roll_The_Dice_v9-2_DE2.aoe2scenario")


if __name__ == "__main__":
    main()
