use std::io;

use pyo3::prelude::*;
use pyo3::pyclass;
use pyo3::types::PyBytes;
use crate::wrap_py;
use crate::types::py_bfp_type::PyBfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;

#[pyclass(module = "bfp_rs.types.le", name = "int16")]
#[derive(Clone)]
pub struct Int16;

impl Parseable for Int16 {
    type Type = i16;

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_py_bfp_type() -> PyBfpType {
        PyBfpType::Int16
    }
    
    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream(stream: &mut ByteStream) -> io::Result<Self::Type> {
        let bytes = stream.get(2)?.try_into().unwrap();
        Ok(i16::from_le_bytes(bytes))
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes(value: &Self::Type) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }
}

wrap_py!(Int16);