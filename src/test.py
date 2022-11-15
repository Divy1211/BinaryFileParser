import zlib

from src.sections.FileData import FileData
from src.sections.Map import Map
from src.sections.Options import Options
from src.sections.ScenarioSections import ScenarioSections
from src.sections.BackgroundImage import BackgroundImage
from src.sections.Cinematics import Cinematics
from src.sections.DataHeader import DataHeader
from src.sections.Diplomacy import Diplomacy
from src.sections.FileHeader import FileHeader
from src.sections.GlobalVictory import GlobalVictory
from src.sections.Messages import Messages
from src.sections.PlayerData2 import PlayerData2
from src.sections.TriggerData import TriggerData
from src.sections.UnitData import UnitData
from src.sections.VariableData import VariableData


def main():
    scx = ScenarioSections.from_file("1_47.aoe2scenario")

    with open("1_47.aoe2scenario", "rb") as file:
        bts = iter(file.read())

    print("file header")
    for cbyte, byte in zip(FileHeader.to_bytes(scx.file_header), bts):
        if cbyte != byte:
            print(cbyte, byte)

    bts = iter(zlib.decompress(bytes(bts), -zlib.MAX_WBITS))

    print("data header")
    for cbyte, byte in zip(DataHeader.to_bytes(scx.data_header), bts):
        if cbyte != byte:
            print(cbyte, byte)

    print("messages")
    for cbyte, byte in zip(Messages.to_bytes(scx.messages), bts):
        if cbyte != byte:
            print(cbyte, byte)

    print("cinematics")
    for cbyte, byte in zip(Cinematics.to_bytes(scx.cinematics), bts):
        if cbyte != byte:
            print(cbyte, byte)

    print("bkg img")
    for cbyte, byte in zip(BackgroundImage.to_bytes(scx.background_image), bts):
        if cbyte != byte:
            print(cbyte, byte)

    print("player data 2")
    for cbyte, byte in zip(PlayerData2.to_bytes(scx.player_data_2), bts):
        if cbyte != byte:
            print(cbyte, byte)

    print("global victory")
    for cbyte, byte in zip(GlobalVictory.to_bytes(scx.global_victory), bts):
        if cbyte != byte:
            print(cbyte, byte)

    print("diplo")
    for cbyte, byte in zip(Diplomacy.to_bytes(scx.diplomacy), bts):
        if cbyte != byte:
            print(cbyte, byte)

    print("options")
    for cbyte, byte in zip(Options.to_bytes(scx.options), bts):
        if cbyte != byte:
            print(cbyte, byte)

    print("map")
    for cbyte, byte in zip(Map.to_bytes(scx.map), bts):
        if cbyte != byte:
            print(cbyte, byte)

    print("unit data")
    for cbyte, byte in zip(UnitData.to_bytes(scx.unit_data), bts):
        if cbyte != byte:
            print(cbyte, byte)

    print("trigger data")
    for cbyte, byte in zip(TriggerData.to_bytes(scx.trigger_data), bts):
        if cbyte != byte:
            print(cbyte, byte)

    print("variable data")
    for cbyte, byte in zip(VariableData.to_bytes(scx.variable_data), bts):
        if cbyte != byte:
            print(cbyte, byte)

    print("file data")
    for cbyte, byte in zip(FileData.to_bytes(scx.file_data), bts):
        if cbyte != byte:
            print(cbyte, byte)

    scx.to_file("first.aoe2scenario")

if __name__ == "__main__":
    main()
