import math as maths
import struct

import pytest

from bfp_rs.errors import ParsingError
from bfp_rs.types.le import (
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, bool8, bool16, bool32, bool64,
    bool128, c_str, nt_str8, nt_str16, nt_str32, nt_str64, nt_str128, Bytes, Str, NtStr,
)


def to_bytes(value: int | float, length: int, signed: bool = False) -> bytes:
    if isinstance(value, int):
        return value.to_bytes(length, byteorder = 'little', signed = signed)
    return struct.pack(f"<{'f' if length == 4 else 'd'}", value)

def test_u8():
    assert u8.from_bytes(to_bytes(42, 1)) == 42
    assert u8.from_bytes(to_bytes(2**(8-1)+1, 1)) == 2**(8-1)+1

    with pytest.raises(ParsingError):
        u8.from_bytes(b"")

def test_u16():
    assert u16.from_bytes(to_bytes(42, 2)) == 42
    assert u16.from_bytes(to_bytes(2**(16-1)+1, 2)) == 2**(16-1)+1

    with pytest.raises(ParsingError):
        u16.from_bytes(b"\x00")

def test_u32():
    assert u32.from_bytes(to_bytes(42, 4)) == 42
    assert u32.from_bytes(to_bytes(2**(32-1)+1, 4)) == 2**(32-1)+1

    with pytest.raises(ParsingError):
        u32.from_bytes(b"\x00" * 3)

def test_u64():
    assert u64.from_bytes(to_bytes(42, 8)) == 42
    assert u64.from_bytes(to_bytes(2**(64-1)+1, 8)) == 2**(64-1)+1

    with pytest.raises(ParsingError):
        u64.from_bytes(b"\x00" * 7)

def test_u128():
    assert u128.from_bytes(to_bytes(42, 16)) == 42
    assert u128.from_bytes(to_bytes(2**(128-1)+1, 16)) == 2**(128-1)+1

    with pytest.raises(ParsingError):
        u128.from_bytes(b"\x00" * 15)

def test_i8():
    assert i8.from_bytes(to_bytes(42, 1, True)) == 42
    assert i8.from_bytes(to_bytes(-2**(8-1), 1, True)) == -2**(8-1)

    with pytest.raises(ParsingError):
        i8.from_bytes(b"")

def test_i16():
    assert i16.from_bytes(to_bytes(42, 2, True)) == 42
    assert i16.from_bytes(to_bytes(-2**(16-1), 2, True)) == -2**(16-1)

    with pytest.raises(ParsingError):
        i16.from_bytes(b"\x00")

def test_i32():
    assert i32.from_bytes(to_bytes(42, 4, True)) == 42
    assert i32.from_bytes(to_bytes(-2**(32-1), 4, True)) == -2**(32-1)

    with pytest.raises(ParsingError):
        i32.from_bytes(b"\x00" * 3)

def test_i64():
    assert i64.from_bytes(to_bytes(42, 8, True)) == 42
    assert i64.from_bytes(to_bytes(-2**(64-1), 8, True)) == -2**(64-1)

    with pytest.raises(ParsingError):
        i64.from_bytes(b"\x00" * 7)

def test_i128():
    assert i128.from_bytes(to_bytes(42, 16, True)) == 42
    assert i128.from_bytes(to_bytes(-2**(128-1), 16, True)) == -2**(128-1)

    with pytest.raises(ParsingError):
        i128.from_bytes(b"\x00" * 15)

def test_f32():
    assert f32.from_bytes(to_bytes(42.0, 4)) == 42.0
    assert f32.from_bytes(to_bytes(float(2**25), 4)) == float(2**25)
    assert maths.isnan(f32.from_bytes(to_bytes(float('nan'), 4)))
    assert maths.isinf(f32.from_bytes(to_bytes(float('inf'), 4)))

    with pytest.raises(ParsingError):
        f32.from_bytes(b"\x00" * 3)

def test_f64():
    assert f64.from_bytes(to_bytes(42.0, 8)) == 42.0
    assert f64.from_bytes(to_bytes(float(2**25), 8)) == float(2**25)
    assert maths.isnan(f64.from_bytes(to_bytes(float('nan'), 8)))
    assert maths.isinf(f64.from_bytes(to_bytes(float('inf'), 8)))

    with pytest.raises(ParsingError):
        f64.from_bytes(b"\x00" * 7)

def test_bool8():
    assert not bool8.from_bytes(to_bytes(0, 1))
    assert bool8.from_bytes(to_bytes(1, 1))
    assert bool8.from_bytes(to_bytes(10, 1))

    with pytest.raises(ParsingError):
        bool8.from_bytes(b"")

def test_bool16():
    assert not bool16.from_bytes(to_bytes(0, 2))
    assert bool16.from_bytes(to_bytes(1, 2))
    assert bool16.from_bytes(to_bytes(10, 2))

    with pytest.raises(ParsingError):
        bool16.from_bytes(b"\x00")

def test_bool32():
    assert not bool32.from_bytes(to_bytes(0, 4))
    assert bool32.from_bytes(to_bytes(1, 4))
    assert bool32.from_bytes(to_bytes(10, 4))

    with pytest.raises(ParsingError):
        bool32.from_bytes(b"\x00" * 3)

def test_bool64():
    assert not bool64.from_bytes(to_bytes(0, 8))
    assert bool64.from_bytes(to_bytes(1, 8))
    assert bool64.from_bytes(to_bytes(10, 8))

    with pytest.raises(ParsingError):
        bool64.from_bytes(b"\x00" * 7)

def test_bool128():
    assert not bool128.from_bytes(to_bytes(0, 16))
    assert bool128.from_bytes(to_bytes(1, 16))
    assert bool128.from_bytes(to_bytes(10, 16))

    with pytest.raises(ParsingError):
        bool128.from_bytes(b"\x00" * 15)

test_str_bytes = b"the quick brown fox jumps over the lazy dog"
test_str = "the quick brown fox jumps over the lazy dog"

def test_c_str():
    assert c_str.from_bytes(test_str_bytes + b"\x00") == test_str
    assert c_str.from_bytes(b"\xff\x00") == "\xff"

    with pytest.raises(ParsingError):
        c_str.from_bytes(test_str_bytes)

def test_bytes_fixed():
    assert Bytes[len(test_str_bytes)].from_bytes(test_str_bytes[::-1]) == test_str_bytes

    with pytest.raises(ParsingError):
        Bytes[len(test_str_bytes)+1].from_bytes(test_str_bytes[::-1])

def test_str_fixed():
    assert Str[len(test_str_bytes)].from_bytes(test_str_bytes) == test_str

    assert Str[1].from_bytes(b"\xff") == "\xff"

    with pytest.raises(ParsingError):
        Str[len(test_str_bytes)+1].from_bytes(test_str_bytes)


def test_nt_str_fixed():
    assert NtStr[len(test_str_bytes) + 1].from_bytes(test_str_bytes+b"\x00") == test_str
    assert NtStr[len(test_str_bytes) + 3].from_bytes(test_str_bytes+b"\x00\x00\x00") == test_str
    assert NtStr[len(test_str_bytes) + 3].from_bytes(test_str_bytes+b"\x00\x01\x02") == test_str
    # null terminator may be omitted
    assert NtStr[len(test_str_bytes)].from_bytes(test_str_bytes) == test_str

    assert NtStr[2].from_bytes(b"\xff\x00") == "\xff"

    with pytest.raises(ParsingError):
        NtStr[len(test_str_bytes) + 1].from_bytes(test_str_bytes)

def test_nt_str8():
    assert nt_str8.from_bytes(to_bytes(len(test_str_bytes) + 1, 1) + test_str_bytes + b"\x00") == test_str
    assert nt_str8.from_bytes(to_bytes(len(test_str_bytes) + 3, 1) + test_str_bytes + b"\x00\x00\x00") == test_str
    assert nt_str8.from_bytes(to_bytes(len(test_str_bytes) + 3, 1) + test_str_bytes + b"\x00\x01\x02") == test_str
    # null terminator may be omitted
    assert nt_str8.from_bytes(to_bytes(len(test_str_bytes), 1) + test_str_bytes) == test_str

    assert nt_str8.from_bytes(to_bytes(2, 1) + b"\xff\x00") == "\xff"

    with pytest.raises(ParsingError):
        nt_str8.from_bytes(to_bytes(len(test_str_bytes) + 1, 1) + test_str_bytes)

def test_nt_str16():
    assert nt_str16.from_bytes(to_bytes(len(test_str_bytes) + 1, 2) + test_str_bytes + b"\x00") == test_str
    assert nt_str16.from_bytes(to_bytes(len(test_str_bytes) + 3, 2) + test_str_bytes + b"\x00\x00\x00") == test_str
    assert nt_str16.from_bytes(to_bytes(len(test_str_bytes) + 3, 2) + test_str_bytes + b"\x00\x01\x02") == test_str
    # null terminator may be omitted
    assert nt_str16.from_bytes(to_bytes(len(test_str_bytes), 2) + test_str_bytes) == test_str

    assert nt_str16.from_bytes(to_bytes(2, 2) + b"\xff\x00") == "\xff"

    with pytest.raises(ParsingError):
        nt_str16.from_bytes(to_bytes(len(test_str_bytes) + 1, 2) + test_str_bytes)

def test_nt_str32():
    assert nt_str32.from_bytes(to_bytes(len(test_str_bytes) + 1, 4) + test_str_bytes + b"\x00") == test_str
    assert nt_str32.from_bytes(to_bytes(len(test_str_bytes) + 3, 4) + test_str_bytes + b"\x00\x00\x00") == test_str
    assert nt_str32.from_bytes(to_bytes(len(test_str_bytes) + 3, 4) + test_str_bytes + b"\x00\x01\x02") == test_str
    # null terminator may be omitted
    assert nt_str32.from_bytes(to_bytes(len(test_str_bytes), 4) + test_str_bytes) == test_str

    assert nt_str32.from_bytes(to_bytes(2, 4) + b"\xff\x00") == "\xff"

    with pytest.raises(ParsingError):
        nt_str32.from_bytes(to_bytes(len(test_str_bytes) + 1, 4) + test_str_bytes)

def test_nt_str64():
    assert nt_str64.from_bytes(to_bytes(len(test_str_bytes) + 1, 8) + test_str_bytes + b"\x00") == test_str
    assert nt_str64.from_bytes(to_bytes(len(test_str_bytes) + 3, 8) + test_str_bytes + b"\x00\x00\x00") == test_str
    assert nt_str64.from_bytes(to_bytes(len(test_str_bytes) + 3, 8) + test_str_bytes + b"\x00\x01\x02") == test_str
    # null terminator may be omitted
    assert nt_str64.from_bytes(to_bytes(len(test_str_bytes), 8) + test_str_bytes) == test_str

    assert nt_str64.from_bytes(to_bytes(2, 8) + b"\xff\x00") == "\xff"

    with pytest.raises(ParsingError):
        nt_str64.from_bytes(to_bytes(len(test_str_bytes) + 1, 8) + test_str_bytes)

def test_nt_str128():
    assert nt_str128.from_bytes(to_bytes(len(test_str_bytes) + 1, 16) + test_str_bytes + b"\x00") == test_str
    assert nt_str128.from_bytes(to_bytes(len(test_str_bytes) + 3, 16) + test_str_bytes + b"\x00\x00\x00") == test_str
    assert nt_str128.from_bytes(to_bytes(len(test_str_bytes) + 3, 16) + test_str_bytes + b"\x00\x01\x02") == test_str
    # null terminator may be omitted
    assert nt_str128.from_bytes(to_bytes(len(test_str_bytes), 16) + test_str_bytes) == test_str

    assert nt_str128.from_bytes(to_bytes(2, 16) + b"\xff\x00") == "\xff"

    with pytest.raises(ParsingError):
        nt_str128.from_bytes(to_bytes(len(test_str_bytes) + 1, 16) + test_str_bytes)
