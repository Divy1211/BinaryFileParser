# BinaryFileParser
BinaryFileParser allows the user to create a binary file format specification in the form of a struct and then use it to
read/write files.

## Installation

On Linux:
```sh
$ python3 -m venv venv
$ source venv/bin/activate
$ pip install binary-file-parser
```

On Windows:
```sh
> py -m venv venv
> venv/Scripts/activate
> pip install binary-file-parser
```

## Getting Started

This is a very basic script to give you an idea of how to use this library. Check out the API Reference (coming soon™) for documentation containing more details on how to use this library.

```py
from binary_file_parser import BaseStruct, Retriever
from binary_file_parser.types import int32, uint64, str32, FixedLenStr

class Spam(BaseStruct):
    file_version: str = Retriever(FixedLenStr[4], default = "1.00")
    creator_name: str = Retriever(str32, default = "bfp")
    saved_timestamp: int = Retriever(uint64, default = 0)
    eggs: int = Retriever(int32, default = 0)

# read a binary file that has the above format
file = Spam.from_file("path/to/file")

# modify the creator name
file.creator_name = "Alian713"
file.eggs = 20

# write the modified data to a new file
file.to_file("path/to/write/to")
```

## A Slightly More Complex Example
The main magic of this library is that:

1. You can use your own structs within another struct
2. Event hooks help you keep any interdependencies in the file structure synchronised:

```py
from __future__ import annotations

from binary_file_parser import BaseStruct, Retriever
from binary_file_parser.types import FixedLenArray, uint32, uint8


class Pixel(BaseStruct):
    red: int = Retriever(uint8, default = 0)
    green: int = Retriever(uint8, default = 0)
    blue: int = Retriever(uint8, default = 0)
    alpha: int = Retriever(uint8, default = 0)

    def __init__(
        self,
        red: int,
        green: int,
        blue: int,
        alpha: int = 0,
    ):
        super().__init__(initialise_defaults = False)
        self.red = red
        self.green = green
        self.blue = blue
        self.alpha = alpha


class Img(BaseStruct):
    @staticmethod
    def _set_width(retriever: Retriever, obj: Img):
        # here Img.pixels.dtype refers to FixedLenArray
        Img.pixels.dtype.length = obj._width

    @staticmethod
    def _set_height(retriever: Retriever, obj: Img):
        # The repeat value defines how many times a single retriever should read data
        Retriever.set_repeat(Img.pixels, obj, obj._height)

    @staticmethod
    def _update_dims(retriever: Retriever, obj: Img):
        # this ensures that when the file is written back, the height and width being written back to file are
        # up to date
        obj._height = obj.height
        Img.pixels.dtype.length = obj._width = obj.width

    _width: int = Retriever(uint32, default = 100, on_read = [_set_width], on_write = [_update_dims])
    _height: int = Retriever(uint32, default = 200, on_set = [_set_height])
    pixels: list[list[Pixel]] = Retriever(
        FixedLenArray[Pixel, 100], default_factory = lambda _, __: [Pixel(0, 0, 0) for _ in range(100)], repeat = 200
    )

    @property
    def width(self) -> int:
        return len(self.pixels[0])

    @property
    def height(self) -> int:
        return len(self.pixels)


# Make a new image from all defaults
a = Img()
# Note: Mutable defaults will be shallow copied, use default_factory when such is not intended
```

## About the Author

If you have any questions, suggestions or feedback regarding the library, feel free to send me a message on discord!

| Author   | Discord       |
|----------|---------------|
| Alian713 | Alian713#0069 |
