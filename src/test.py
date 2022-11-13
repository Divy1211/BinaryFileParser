import zlib

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.sections.AoE2Scenario import AoE2Scenario
from src.sections.BackgroundImage import BackgroundImage
from src.sections.Cinematics import Cinematics
from src.sections.DataHeader import DataHeader
from src.sections.FileHeader import FileHeader
from src.sections.Messages import Messages


def manual_create_test():
    with open("test.aoe2scenario", "wb") as file:
        file.write(b"1.47")
        file.write(int.to_bytes(0, length = 4, byteorder = "little", signed = False))
        file.write(int.to_bytes(6, length = 4, byteorder = "little", signed = True))
        file.write(int.to_bytes(1610675127, length = 4, byteorder = "little", signed = False))
        file.write(int.to_bytes(6, length = 4, byteorder = "little", signed = False)+b"funny\x00")
        file.write(int.to_bytes(2, length = 4, byteorder = "little", signed = False))
        n = 6
        file.write(int.to_bytes(n, length = 4, byteorder = "little", signed = False))
        for i in range(n):
            file.write(int.to_bytes(i+2, length = 4, byteorder = "little", signed = False))
        file.write(int.to_bytes(9, length = 4, byteorder = "little", signed = False)+b"Alian713\x00")
        file.write(int.to_bytes(420, length = 4, byteorder = "little", signed = False))

def main():
    scx = AoE2Scenario.from_file("default0.aoe2scenario")

    # igen = IncrementalGenerator.from_file("default0.aoe2scenario")
    #
    # fh = FileHeader.from_generator(igen)
    print("FILE HEADER")

    print(scx.file_header.file_version_str)
    print(scx.file_header.header_len)
    print(scx.file_header.savable)
    print(scx.file_header.timestamp_of_last_save)
    print(scx.file_header.scenario_instructions)
    print(scx.file_header.num_players)
    print(scx.file_header.unknown1)
    print(scx.file_header.unknown2)
    print(scx.file_header.unknowns)
    print(scx.file_header.creator)
    print(scx.file_header.num_triggers)

    # file_content = igen.get_remaining_bytes()
    #
    # igen = IncrementalGenerator.from_bytes(zlib.decompress(file_content, -zlib.MAX_WBITS))
    #
    # dh = DataHeader.from_generator(igen)
    print("DATA HEADER")

    print(scx.data_header.next_unit_id)
    print(scx.data_header.version)
    print(scx.data_header.tribe_names)
    print(scx.data_header.string_table_player_names)
    print(scx.data_header.player_data_1[0].active)
    print(scx.data_header.player_data_1[0].human)
    print(scx.data_header.player_data_1[0].civilization)
    print(scx.data_header.player_data_1[0].architecture_set)
    print(scx.data_header.player_data_1[0].cty_mode)
    print(scx.data_header.lock_civs)
    print(scx.data_header.unknown)
    print(scx.data_header.filename)

    # msgs = Messages.from_generator(igen)
    print("MESSAGES")

    print(scx.messages.instructions_str_id)
    print(scx.messages.hints_str_id)
    print(scx.messages.victory_str_id)
    print(scx.messages.loss_str_id)
    print(scx.messages.history_str_id)
    print(scx.messages.scouts_str_id)
    print(scx.messages.instructions)
    print(scx.messages.hints)
    print(scx.messages.victory)
    print(scx.messages.loss)
    print(scx.messages.history)
    print(scx.messages.scouts)

    # c = Cinematics.from_generator(igen)
    print("CINEMATICS")

    print(scx.cinematics.pregame)
    print(scx.cinematics.victory)
    print(scx.cinematics.loss)

    # bkgimg = BackgroundImage.from_generator(igen)
    print("BACKGROUND IMG")

    print(scx.background_image.filename)
    print(scx.background_image.version)
    print(scx.background_image.width)
    print(scx.background_image.height)
    print(scx.background_image.orientation)
    print(scx.background_image.info)
    print(scx.background_image.image)


    fhb = FileHeader.to_bytes(scx.file_header)
    bytes_ = b""
    bytes_ += DataHeader.to_bytes(scx.data_header)
    bytes_ += Messages.to_bytes(scx.messages)
    bytes_ += Cinematics.to_bytes(scx.cinematics)
    bytes_ += BackgroundImage.to_bytes(scx.background_image)

    with open("default0.aoe2scenario", "rb") as file:
        bts = iter(file.read())

    for fhbyte, byte in zip(fhb, bts):
        if fhbyte != byte:
            print(fhbyte, byte)

    rbytes = zlib.decompress(bytes(bts), -zlib.MAX_WBITS)

    for cbyte, byte in zip(bytes_, rbytes):
        if cbyte != byte:
            print(cbyte, byte)

    print("woo")

if __name__ == "__main__":
    main()
