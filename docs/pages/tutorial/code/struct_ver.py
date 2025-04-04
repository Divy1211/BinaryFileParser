from typing import cast

from bfp_rs import BaseStruct, Retriever, Version, ByteStream, ret
from bfp_rs.combinators import set_repeat
from bfp_rs.errors import VersionError
from bfp_rs.types.le import Bytes, u16, u32, Tail, u8


class Packet(BaseStruct):
    # @formatter:off
    timestamp_seconds: int       = Retriever(u32,      default = 0)
    timestamp_micro_seconds: int = Retriever(u32,      default = 0)
    captured_length: int         = Retriever(u32,      default = 0, on_read = lambda: [set_repeat(ret(Packet.data)).from_(Packet.captured_length)])
    original_length: int         = Retriever(u32,      default = 0)
    data: list[bytes]            = Retriever(Bytes[1], default = b"\x00")
    # @formatter:on

class PcapHeader(BaseStruct):
    # @formatter:off
    magic_number: bytes     = Retriever(Bytes[4],                                   default = b"\xa1\xb2\xc3\xd4")
    version_major: int      = Retriever(u16,                                        default = 2)
    version_minor: int      = Retriever(u16,                                        default = 4)
    timezone: int           = Retriever(u32,                                        default = 0)
    timestamp_accuracy: int = Retriever(u32,                                        default = 0)
    snap_length: int        = Retriever(u32,                                        default = 0)
    link_layer_type: int    = Retriever(u32,                                        default = 1)

    interface_index: int    = Retriever(u32,      min_ver = Version(2, 4, 1), default = 0)
    protocol: int           = Retriever(u16,      min_ver = Version(2, 4, 1), default = 0)
    packet_type: int        = Retriever(u8,       min_ver = Version(2, 4, 1), default = 0)
    padding: bytes          = Retriever(Bytes[1], min_ver = Version(2, 4, 1), default = b"\x00")
    # @formatter:on

class PcapFile(BaseStruct):
    header: PcapHeader      = Retriever(PcapHeader,   default_factory = PcapHeader)
    packets: list[Packet]   = Retriever(Tail[Packet], default_factory = lambda _ver: [])

    @classmethod
    def _get_version(cls, stream: ByteStream, ver: Version = Version(0)) -> Version:
        bytes_ = stream.peek(8)
        magic, ver_bytes = bytes_[:4], bytes_[4:]
        major, minor = u16.from_bytes(ver_bytes[:2]), u16.from_bytes(ver_bytes[2:])

        if major != 2 or minor != 4:
            raise VersionError(f"Unrecognised version v{major}.{minor} for Pcap file")

        if magic == b"\xd4\xc3\xb2\xa1":
            return Version(2, 4)
        if magic == b"\xcd\x34\xb2\xa1":
            return Version(2, 4, 1)

        raise VersionError(f"Unrecognised magic_number {magic[::-1]!r} for pcap file")

    def __new__(cls, ver: Version = Version(2, 4), init_defaults: bool = True, **retriever_inits):
        self = cast(PcapFile, super().__new__(cls, ver, init_defaults, **retriever_inits))
        if ver == Version(2, 4, 1):
            self.header.magic_number = b"\xa1\xb2\xcd\x34"
        return self

test = PcapFile.from_file(r"ipv4frags.pcap")
print(test.ver)

print(test.header.protocol)
