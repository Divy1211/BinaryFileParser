use pyo3::prelude::*;
use pyo3::pyclass;
use pyo3::types::PyBytes;

use crate::{def_num_type_le, wrap_py};
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;
use crate::types::version::Version;


def_num_type_le!(UInt8, "uint8", u8, 1);
def_num_type_le!(UInt16, "uint16", u16, 2);
def_num_type_le!(UInt32, "uint32", u32, 4);
def_num_type_le!(UInt64, "uint64", u64, 8);
def_num_type_le!(UInt128, "uint128", u128, 16);

def_num_type_le!(Int8, "int8", i8, 1);
def_num_type_le!(Int16, "int16", i16, 2);
def_num_type_le!(Int32, "int32", i32, 4);
def_num_type_le!(Int64, "int64", i64, 8);
def_num_type_le!(Int128, "int128", i128, 16);
