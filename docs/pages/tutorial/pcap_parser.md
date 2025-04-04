## About this Tutorial

This tutorial will walk you through constructing your own PCAP parser using BFP! You can read more about PCAP files themselves [here](https://www.netresec.com/?page=Blog&month=2022-10&post=What-is-a-PCAP-file)

## Obtain a Sample File

Whenever you want to make a new parser for a file format, it is always a good idea to get a few small file samples so that you can test the parser as you code. For the purposes of this tutorial, we'll use the [ipv4frags.pcap](https://wiki.wireshark.org/uploads/__moin_import__/attachments/SampleCaptures/ipv4frags.pcap) file from the [wireshark wiki](https://wiki.wireshark.org/samplecaptures#general--unsorted)

## Defining the Top Level Struct

We begin by defining a new class `PcapFile` which inherits from [`BaseStruct`](../../reference/types/base_struct/):

```py
from bfp_rs import BaseStruct

class PcapFile(BaseStruct):
    ...
```

This is the class where we will define all the fields for the format, and it will also be used to create instances when BFP parses a file

## Pcap Header

Next, we'll define the header for the PCAP file according to its specification:

```py
from bfp_rs import BaseStruct, Retriever
from bfp_rs.types.le import Bytes, u16, u32

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
```

A couple of things are new here, so let's go over them one by one:

1. [`Retriever`](../../reference/retrievers/retriever/) - This defines a new property that this struct will read or write during parsing. This process is also often known as serialization (going from struct to binary representation) and deserialization (going from binary to struct representation)
2. Each retriever takes a type which specifies how it will interpret the bytes from the file and assign them to the properties. This is the first argument to the constructor. 

    !!! tip "Use the API Reference!"
        If you can't guess what each of the types do, you can always check out the API reference for what the types (e.g.[`Bytes`](../../reference/types/le/bytes/)) in each `Retriever` do. You can do the same for any other types here that you may not recognise!

3. Each `Retriever` optionally accepts a `default` argument - this is used if you ever decide to create a new instance of your struct in code. This may not always be required, but it is good practice to specify it anyway.
4. The properties are wrapped in `@formatter:off` and `@formatter:on` so that the vertical alignment of the types and arguments does not get messed up. This is the recommended way to write BFP structs, as it maintains readability and allows you to determine a struct's schema at a glance.
5. At this point, you can create a **default** `PcapHeader` **instance** of your own:
   ```py
   header = PcapHeader()
   ```

But we're here to parse files into instances, not create our own instances! So let's add the header definition to the `PcapFile` struct:

```py
from bfp_rs import BaseStruct, Retriever

class PcapFile(BaseStruct):
    header: PcapHeader      = Retriever(PcapHeader,   default_factory = lambda ver: PcapHeader(ver))
```

- Yes, we have just passed `PcapHeader` as the type for `Retriever` to serialize! Every `BaseStruct` subclass is a valid type to be provided to `Retriever` - this is part of what makes BFP serialization powerful - you can easily define nested structs and the parsing will just work!
- Notice that instead of using `default`, we've used a `default_factory`. This is the recommended method to provide default values for any mutable types in structs.
- This function is called with a `ver: Version` and it must return an instance of `PcapHeader` which will be assigned to `header` when `PcapFile` is default initialized (when you use `PcapFile()` in code)

!!! question "Why do we have two different syntaxes for defining defaults?"
    What stops us from using `default = PcapHeader()`? There are two reasons:

    - The `PcapHeader` instance has no way to know what struct version it is in (this will make more sense in the next section on [Struct Versioning](../../tutorial/struct_versioning/) in this tutorial).
    - If done this way, every default instance of a `PcapFile` would point to the same `PcapHeader`. Read more about this [here](https://stackoverflow.com/questions/64136035/why-should-i-set-a-function-list-argument-as-empty-or-none-instead-of-using-a-no).


At this point, the full code looks like:

```py
from bfp_rs import BaseStruct, Retriever
from bfp_rs.types.le import Bytes, u16, u32

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
    # The lambda in the default_facotry was for illustration purposes - since every BaseStruct subclass' constructor
    # accepts a version argument and returns an instance of... itself, they're valid functions to be given to 
    # default_factory! 
```

You can try to read the pcap file now and see if all goes well:

```py
pcap = PcapFile.from_file(r"/path/to/ipv4frags.pcap")
```

We get an error:

```pycon
Traceback (most recent call last):
  File "/path/to/code.py", line 35, in <module>
    PcapFile.from_file(r"ipv4frags.pcap")
errors.ParsingError: 2966 bytes are left after parsing all retrievers successfully
```

So what happened?

When BFP parses a file into a struct, it expects all the data to be fully consumed. When this does not happen, it will raise a `ParsingError`.

So how do we test the definition we have so far? We can set `strict = False` in the `from_file` call, and BFP will ignore the unused bytes at the end:

```py
pcap = PcapFile.from_file(r"/path/to/ipv4frags.pcap", strict = False)
print(pcap.header.magic_number) # prints b"\xa1\xb2\xc3\xd4"
```

## Completing the Definition

Now we need to be able to parse a list of packets, so let's define a struct for it:

```py
from bfp_rs import BaseStruct, Retriever
from bfp_rs.types.le import Bytes, u32
class Packet(BaseStruct):
    # @formatter:off
    timestamp_seconds: int       = Retriever(u32,      default = 0)
    timestamp_micro_seconds: int = Retriever(u32,      default = 0)
    captured_length: int         = Retriever(u32,      default = 0)
    original_length: int         = Retriever(u32,      default = 0)
    data: bytes                  = Retriever(Bytes[1], default = b"\x00")
    # @formatter:on
```

This is almost correct, but notice that currently the `data` field is only 1 byte long! We need some way to tell BFP that `data` is actually a list of bytes with length `captured_length` number of times. We can use a [`set_repeat`](../../reference/combinators/set_repeat_builder/#bfp_rs.combinators.set_repeat_builder.set_repeat) combinator to achieve this:

```py
from bfp_rs import BaseStruct, Retriever, ret
from bfp_rs.types.le import Bytes, u32
from bfp_rs.combinators import set_repeat

class Packet(BaseStruct):
    # @formatter:off
    timestamp_seconds: int       = Retriever(u32,      default = 0)
    timestamp_micro_seconds: int = Retriever(u32,      default = 0)
    captured_length: int         = Retriever(u32,      default = 0, on_read = lambda: [set_repeat(ret(Packet.data)).from_(Packet.captured_length)])
    original_length: int         = Retriever(u32,      default = 0)
    data: list[bytes]            = Retriever(Bytes[1], default = b"\x00")
    # @formatter:on
```

We're almost there! Now let's add the `Packet` definition to the `PcapFile` and we'll be done:

```py
from bfp_rs import BaseStruct, Retriever
from bfp_rs.types.le import Tail

class PcapFile(BaseStruct):
    header: PcapHeader      = Retriever(PcapHeader,   default_factory = PcapHeader)
    packets: list[Packet]   = Retriever(Tail[Packet], default_factory = lambda _ver: [])
```

Here, [`Tail`](../../reference/types/le/tail/) reads a list of it's given type until the end of file

Once again notice that we can simply pass a struct (or any other type in BFP) to a container type like `Tail`. This composition of types is at the heart of BFP's declarative style and ease of use.

We're now ready to remove `strict = False` and parse the whole file:

```py
pcap = PcapFile.from_file(r"ipv4frags.pcap")
```

Yippee!! You've just created your first serialization file format using BFP!

If you now make edits to this file programmatically, you can save it to a new file:

```py
PcapFile.to_file(r"ipv4frags.pcap", pcap)
```

## The Code

Here's the completed code in all it's glory:

```py
from bfp_rs import BaseStruct, Retriever, ret
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

pcap = PcapFile.from_file(r"ipv4frags.pcap")
print(len(pcap.packets)) # prints 3

# edit pcap to remove the last packet
pcap.packets = pcap.packets[:2]

PcapFile.to_file(r"ipv4frags.pcap", pcap)
```