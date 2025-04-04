## What is Struct Versioning

If a binary file format changes over the course of development, you can version the structure to conditionally parse or skip parsing certain properties, which is useful for maintaining parser compatibility with older files. When coming up with a new struct for a new project, it is highly recommended that some sort of versioning information be included at the start of the file itself as a future proofing measure. For the sake of this tutorial, we'll look at how we'd modify the `PcapFile` definition from the previous tutorial to include some extra fields conditionally, based on the `magic_number` property

## The Modified PCAP

One variation of the PCAP file is where the header includes the following extra information:

```py
class PcapHeader(BaseStruct):
    # @formatter:off
    magic_number: bytes     = Retriever(Bytes[4], default = b"\xa1\xb2\xc3\xd4")
    version_major: int      = Retriever(u16,      default = 2)
    version_minor: int      = Retriever(u16,      default = 4)
    timezone: int           = Retriever(u32,      default = 0)
    timestamp_accuracy: int = Retriever(u32,      default = 0)
    snap_length: int        = Retriever(u32,      default = 0)
    link_layer_type: int    = Retriever(u32,      default = 1)

    # new fields:
    interface_index: int    = Retriever(u32,       default = 0)
    protocol: int           = Retriever(u16,       default = 0)
    packet_type: int        = Retriever(u8,        default = 0)
    padding: bytes          = Retriever(Bytes[1],  default = b"\x00")
    # @formatter:on
```

These extra fields are included only if the `magic_number` property is `b"\xa1\xb2\xcd\x34"` (notice the last two bytes) instead of `b"\xa1\xb2\xc3\xd4"`

But if we just include the extra fields like this, it will break the parser for any files which do not contain that extra piece of information in the header. We need some way to signal to BFP that these should be [de]serialized conditionally. Let's specify a [`min_ver`](../../reference/retrievers/retriever/#bfp_rs.retrievers.retriever.Retriever.__new__) to tell BFP a lower bound on the struct version where these properties are included:

```py
class PcapHeader(BaseStruct):
    # @formatter:off
    magic_number: bytes     = Retriever(Bytes[4],                             default = b"\xa1\xb2\xc3\xd4")
    version_major: int      = Retriever(u16,                                  default = 2)
    version_minor: int      = Retriever(u16,                                  default = 4)
    timezone: int           = Retriever(u32,                                  default = 0)
    timestamp_accuracy: int = Retriever(u32,                                  default = 0)
    snap_length: int        = Retriever(u32,                                  default = 0)
    link_layer_type: int    = Retriever(u32,                                  default = 1)
    
    interface_index: int    = Retriever(u32,      min_ver = Version(2, 4, 1), default = 0)
    protocol: int           = Retriever(u16,      min_ver = Version(2, 4, 1), default = 0)
    packet_type: int        = Retriever(u8,       min_ver = Version(2, 4, 1), default = 0)
    padding: bytes          = Retriever(Bytes[1], min_ver = Version(2, 4, 1), default = 0)
    # @formatter:on
```

!!! note "v2.4.1"
    
    Such a version **number** does not actually exist, it was chosen arbitrarily since the real format uses the magic number for the purpose of versioning.

!!! tip "Maximum Versions"

    We can similarly also specify a `max_ver` which sets an upper bound on the struct version where these properties are included.

## Determining the Struct Version During Deserialization

Now when we deserialize a file, we also need to set a struct version for BFP to be able to make use of the versioning information we just added above. This is done by overriding the [`_get_verison`](../../reference/types/base_struct/#bfp_rs.types.base_struct.BaseStruct._get_version) function in a struct:

```py
class PcapFile(BaseStruct):
    header: PcapHeader      = Retriever(PcapHeader,   default_factory = PcapHeader)
    packets: list[Packet]   = Retriever(Tail[Packet], default_factory = lambda _ver: [])

    @classmethod
    def _get_version(cls, stream: ByteStream, ver: Version = Version(0)):
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
```

This function must determine a struct version from the [`ByteStream`](../../reference/types/byte_stream/) and return it, or raise an error. There are a few things to note:

- We raise an error if we see any other version than `v2.4` since that's the only one we know how to parse
- We also raise an error if the magic number is not one of the ones we recognise
- Arbitrarily assign a different higher version when magic is set to `b"\xa1\xb2\xcd\x34"`. Note that since this file is stored in little endian and `ByteStream` returns raw bytes, we need to reverse the order of the bytes before making the comparison.
- When a version is set this way, it is recursively also set for all sub-structs unless they also override `_get_version` for themselves
- When you override the `_get_version` function in a sub struct, the second argument `ver` is given the version of the parent struct

!!! warning "`peek` vs `get`"

    As a rule of thumb, you should never consume any bytes from the `stream` in `_get_version`, always use `stream.peek(n)` over `stream.get(n)`

With this, we're done! Now you'll be able to [de]serialize PCAP files with the special `magic_number`! Notably, you can still [de]serialize the standard PCAP file as well, since the extra fields in the `PcapHeader` are only [de]serialized when the special `magic_number` is present.

You can check if you can still read the old format:

```py
test = PcapFile.from_file(r"ipv4frags.pcap")
print(test.ver) # prints v2.4
```

Additionally, if you tried accessing one of the extra fields in a version where it is not supported, you'll be met with an error:

```py
print(test.header.protocol)
```

Yields:

```
Traceback (most recent call last):
  File "/path/to/code.py", line 63, in <module>
    print(test.header.protocol)
errors.VersionError: 'protocol' is not supported in struct version v2.4
```

## Default Initialization

While the definition above can properly [de]serialize data from existing files, it will not be able to correctly create default instances with the new version of the struct. To properly support default initialization, we must also tell BFP which version to use for this struct when it is default initialized. This can be done by overriding the [`__new__`](../../reference/types/base_struct/#bfp_rs.types.base_struct.BaseStruct.__new__) function in a struct:

```py
from typing import cast

class PcapFile(BaseStruct):
    header: PcapHeader      = Retriever(PcapHeader,   default_factory = PcapHeader)
    packets: list[Packet]   = Retriever(Tail[Packet], default_factory = lambda _ver: [])

    @classmethod
    def _get_version(cls, stream: ByteStream, ver: Version = Version(0)):
        ...

    def __new__(cls, ver: Version = Version(2, 4), init_defaults: bool = True, **retriever_inits):
        self = cast(PcapFile, super().__new__(cls, ver, init_defaults, **retriever_inits))
        if ver == Version(2, 4, 1):
            self.header.magic_number = b"\xa1\xb2\xcd\x34"
        return self
```

Let's break down what's going on here:

1. We're changing the default of `ver` to `Version(2, 4)` in the constructor. This means when someone creates a default instance using `PcapFile()`, it's version will be initialized to `v2.4`
2. Since we want people to be able to create defaults of a different version by using `PcapFile(ver = Version(2, 4, 1))`, we need to fix the `magic_number` manually so that it is serialized correctly when we write this new instance to a file.

!!! tip "Different Defaults Across Versions"

    You can use `min_ver` and `max_ver` to make it so that a different property is used for the default initialization in each version, which allows specifying a different default:
    ```py
    class PcapHeader(BaseStruct):
        # @formatter:off
        magic_number_24: bytes      = Retriever(Bytes[4],                             max_ver = Version(2, 4)   default = b"\xa1\xb2\xc3\xd4")
        magic_number_241: bytes     = Retriever(Bytes[4], min_ver = Version(2, 4, 1)                            default = b"\xa1\xb2\xcd\x34")
    ```
    This means that we no longer need to have special logic in the constructor to fix it for us, but it has the downside that it is more awkward to use, since the property you need to access now changes depending on the file version - BFP offers a way to combine such properties into one using a [`RetrieverCombiner`](../../reference/retrievers/retriever_combiner/). Using these is covered in the [Advanced Retrievers](../../tutorial/advanced_retrievers/) section of this tutorial.

    Note that the choice to implement correct defaults for different versions with different properites or with special logic in the constructor is up to the struct designer, neither method is preferred over the other by BFP itself.


!!! note "PCAP Variants"

    There are more variations of the PCAP format based different `magic_number`s. Some very old files may also have a version number of `2.3` or even `2.2`. You can read about other PCAP file variations [here](https://wiki.wireshark.org/Development/LibpcapFileFormat?utm_source=chatgpt.com#variants). Implementing a parser that can work with all these variations is left as an exercise for the reader.

## The Code

Here's the completed code in all it's glory:

```py
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
```