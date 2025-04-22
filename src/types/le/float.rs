use pyo3::prelude::*;
use pyo3::pyclass;
use pyo3::types::PyBytes;

use crate::{def_num_type_le, wrap_py};
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;
use crate::types::version::Version;

def_num_type_le!(Float32, "float32", f32, 4);
def_num_type_le!(Float64, "float64", f64, 8);

