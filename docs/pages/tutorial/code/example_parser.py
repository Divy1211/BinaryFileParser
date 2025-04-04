from bfp_rs import BaseStruct, Retriever, Version, ByteStream, ret, set_mut, borrow_mut
from bfp_rs.combinators import set_repeat
from bfp_rs.types.le import Bytes, u16, u32, Tail


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
    magic_number: bytes     = Retriever(Bytes[4],     default = b"\xa1\xb2\xc3\xd4")
    version_major: int      = Retriever(u16,          default = 2)
    version_minor: int      = Retriever(u16,          default = 4)
    timezone: int           = Retriever(u32,          default = 0)
    timestamp_accuracy: int = Retriever(u32,          default = 0)
    snap_length: int        = Retriever(u32,          default = 0)
    link_layer_type: int    = Retriever(u32,          default = 1)
    # @formatter:on

class PcapFile(BaseStruct):
    header: PcapHeader      = Retriever(PcapHeader,   default_factory = PcapHeader)
    packets: list[Packet]   = Retriever(Tail[Packet], default_factory = lambda _ver: [])

    @classmethod
    def _get_version(cls, stream: ByteStream, ver: Version = Version(0)):
        ver_bytes = stream.peek(8)[4:]
        return Version(u16.from_bytes(ver_bytes[:2]), u16.from_bytes(ver_bytes[2:]))

test = PcapFile.from_file(r"ipv4frags.pcap")
