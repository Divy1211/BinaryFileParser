import zlib

from src.generators.IncrementalGenerator import IncrementalGenerator
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
    igen = IncrementalGenerator.from_file("default0.aoe2scenario")

    fh = FileHeader.from_generator(igen)
    print("FILE HEADER")

    print(fh.file_version_str)
    print(fh.header_len)
    print(fh.savable)
    print(fh.timestamp_of_last_save)
    print(fh.scenario_instructions)
    print(fh.num_players)
    print(fh.unknown1)
    print(fh.unknown2)
    print(fh.unknowns)
    print(fh.creator)
    print(fh.num_triggers)

    file_content = igen.get_remaining_bytes()

    igen = IncrementalGenerator.from_bytes(zlib.decompress(file_content, -zlib.MAX_WBITS))

    dh = DataHeader.from_generator(igen)
    print("DATA HEADER")

    print(dh.next_unit_id)
    print(dh.version)
    print(dh.tribe_names)
    print(dh.string_table_player_names)
    print(dh.player_data_1[0].active)
    print(dh.player_data_1[0].human)
    print(dh.player_data_1[0].civilization)
    print(dh.player_data_1[0].architecture_set)
    print(dh.player_data_1[0].cty_mode)
    print(dh.lock_civs)
    print(dh.unknown)
    print(dh.filename)

    msgs = Messages.from_generator(igen)
    print("MESSAGES")

    print(msgs.instructions_str_id)
    print(msgs.hints_str_id)
    print(msgs.victory_str_id)
    print(msgs.loss_str_id)
    print(msgs.history_str_id)
    print(msgs.scouts_str_id)
    print(msgs.instructions)
    print(msgs.hints)
    print(msgs.victory)
    print(msgs.loss)
    print(msgs.history)
    print(msgs.scouts)

    c = Cinematics.from_generator(igen)
    print("CINEMATICS")

    print(c.pregame)
    print(c.victory)
    print(c.loss)

    bkgimg = BackgroundImage.from_generator(igen)
    print("BACKGROUND IMG")

    print(bkgimg.filename)
    print(bkgimg.version)
    print(bkgimg.width)
    print(bkgimg.height)
    print(bkgimg.orientation)
    print(bkgimg.info)
    print(bkgimg.image)


    fhb = fh.to_bytes(fh)
    bytes_ = b""
    bytes_ += dh.to_bytes(dh)
    bytes_ += msgs.to_bytes(msgs)
    bytes_ += c.to_bytes(c)
    bytes_ += bkgimg.to_bytes(bkgimg)

    with open("default0.aoe2scenario", "rb") as file:
        bts = iter(file.read())

    for fhbyte, byte in zip(fhb, bts):
        if fhbyte != byte:
            print(fhbyte, byte)

    rbytes = zlib.decompress(bytes(bts), -zlib.MAX_WBITS)

    for cbyte, byte in zip(bytes_, rbytes):
        if cbyte != byte:
            print(cbyte, byte)

    print('WO')

if __name__ == "__main__":
    main()
