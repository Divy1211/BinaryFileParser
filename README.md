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
    file_version: str = Retriever(FixedLenStr[4], default = 0)
    creator_name: str = Retriever(str32, default = 0)
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

## About the Author

If you have any questions, suggestions or feedback regarding the library, feel free to send me a message on discord!

| Author   | Discord       |
|----------|---------------|
| Alian713 | Alian713#0069 |
