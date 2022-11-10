# with open("test.aoe2scenario", "wb") as file:
#     file.write(b"1.47")
#     file.write(int.to_bytes(0, length = 4, byteorder = "little", signed = False))
#     file.write(int.to_bytes(6, length = 4, byteorder = "little", signed = True))
#     file.write(int.to_bytes(1610675127, length = 4, byteorder = "little", signed = False))
#     file.write(int.to_bytes(1, length = 4, byteorder = "little", signed = False)+b"\x00")
#     file.write(int.to_bytes(2, length = 4, byteorder = "little", signed = False))
#     n = 10_000
#     file.write(int.to_bytes(n, length = 4, byteorder = "little", signed = False))
#     for i in range(n):
#         file.write(int.to_bytes(i, length = 4, byteorder = "little", signed = False))
#     file.write(int.to_bytes(9, length = 4, byteorder = "little", signed = False)+b"Alian713\x00")
#     file.write(int.to_bytes(420, length = 4, byteorder = "little", signed = False))

from src.generators.IncrementalGenerator import IncrementalGenerator
from src.sections.FileHeader import FileHeader

igen = IncrementalGenerator.from_file("test.aoe2scenario")

header = FileHeader(igen)
print(header.file_version_str)
print(header.savable)
print(header.timestamp_of_last_save)
print(header.scenario_instructions)
print(header.player_count)
print(header.unknowns)
print(header.creator_name)
print(header.trigger_count)

header.write_to_file("test2.aoe2scenario")

with open("test.aoe2scenario", "rb") as file:
    read_from = file.read()

with open("test2.aoe2scenario", "rb") as file:
    write_to = file.read()

print(f"{read_from == write_to = }")
