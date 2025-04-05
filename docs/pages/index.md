# BinaryFileParser

A blazingly fast python library written in rust for serializing/deserializing python objects to/from binary files. BFP syntax is declarative and very versatile, allowing the user to specify file formats in a very readable and concise manner.

## Installation

```
pip install --pre binary-file-parser
```

Note: whilst v0.3 is currently in alpha, it is the version new users are recommended to use, as the API has been
significantly reworked from v0.2 and will become the standard once it matures and stabilises. Minor breakage moving
forward from v0.3 may still be introduced as the API is fully ironed out.

## Development

1. Have [rust](https://www.rust-lang.org/) and [python](https://www.python.org/) installed
2. Create an empty directory and `cd` into it
3. Clone: `git clone -b bfp-rs https://github.com/Divy1211/BinaryFileParser.git`
4. Create a venv: `python -m venv venv` and activate it.
5. Install maturin `pip install maturin`
6. Do a build:
   - dev: `maturin develope`
   - release: `maturin develop --release`
7. Play

## Getting Started

View the BFP tutorial [here](https://divy1211.github.io/BinaryFileParser/tutorial/pcap_parser/) which will walk you through
constructing a PCAP file parser

## About the Author

If you have any questions, suggestions or feedback regarding the library, feel free to send me a message on discord!

| Author   | Discord       |
|----------|---------------|
| Alian713 | Alian713#0069 |
