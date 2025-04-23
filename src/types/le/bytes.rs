use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.types.le", name = "Bytes")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bytes {
    pub len: usize,
}

impl Parseable for Bytes {
    type Type = Vec<u8>;

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream(&self, stream: &mut ByteStream, _ver: &Version) -> PyResult<Self::Type> {
        let mut bytes = stream.get(self.len)?.to_vec();
        bytes[..].reverse();
        Ok(bytes)
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()> {
        buffer.extend(value.iter().rev());
        Ok(())
    }
}


#[pymethods]
impl Bytes {
    #[pyo3(name = "to_bytes")]
    fn to_bytes_py(slf: PyRef<Self>, value: <Self as Parseable>::Type) -> PyResult<Bound<PyBytes>> {
        let bytes = slf.to_bytes(&value)?;
        Ok(PyBytes::new_bound(slf.py(), &bytes))
    }

    #[pyo3(name = "from_stream", signature = (stream, ver = Version::new(vec![0,])))]
    fn from_stream_py(slf: PyRef<Self>, stream: &mut ByteStream, ver: Version) -> PyResult<<Self as Parseable>::Type> {
        Ok(slf.from_stream(stream, &ver)?)
    }

    #[pyo3(name = "from_file")]
    fn from_file_py(slf: PyRef<Self>, filepath: &str) -> PyResult<<Self as Parseable>::Type> {
        Ok(slf.from_file(filepath)?)
    }
    #[pyo3(name = "from_bytes", signature = (bytes, ver = Version::new(vec![0,])))]
    fn from_bytes_py(slf: PyRef<Self>, bytes: &[u8], ver: Version) -> PyResult<<Self as Parseable>::Type> {
        Ok(slf.from_bytes(bytes, &ver)?)
    }
    #[pyo3(name = "to_file")]
    fn to_file_py(slf: PyRef<Self>, filepath: &str, value: <Self as Parseable>::Type) -> PyResult<()> {
        Ok(slf.to_file(filepath, &value)?)
    }

    #[classmethod]
    fn __class_getitem__(_cls: &Bound<PyType>, len: usize) -> BfpType {
        BfpType::Bytes(Bytes { len })
    }
}
