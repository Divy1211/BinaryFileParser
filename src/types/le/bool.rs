use pyo3::prelude::*;
use pyo3::pyclass;
use pyo3::types::PyBytes;

use crate::{def_bool_type_le, wrap_py};
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;
use crate::types::version::Version;

def_bool_type_le!(Bool8, "bool8", u8, 1);
def_bool_type_le!(Bool16, "bool16", u16, 2);
def_bool_type_le!(Bool32, "bool32", u32, 4);
def_bool_type_le!(Bool64, "bool64", u64, 8);
def_bool_type_le!(Bool128, "bool128", u128, 16);
