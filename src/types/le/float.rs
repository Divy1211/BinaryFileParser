use std::io;

use pyo3::prelude::*;
use pyo3::pyclass;
use pyo3::types::PyBytes;
use crate::{def_num_type_le, wrap_py};
use crate::types::py_bfp_type::PyBfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;

def_num_type_le!(Float32, "float32", f32, 4);
def_num_type_le!(Float64, "float64", f64, 8);

