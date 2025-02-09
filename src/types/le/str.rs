use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::le::encoding::Encoding;
use crate::types::le::size::Size;
use crate::types::le::utils::{str_from_bytes, str_to_bytes};
use crate::types::parseable::Parseable;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.types.le", name = "Str")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Str {
    pub len: Size,
    pub enc1: Encoding,
    pub enc2: Option<Encoding>,
}

impl Parseable for Str {
    type Type = String;

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream(&self, stream: &mut ByteStream, _version: &Version) -> std::io::Result<Self::Type> {
        let len = self.len.from_stream(stream, _version)?;
        let bytes = stream.get(len)?;
        str_from_bytes(bytes, &self.enc1, &self.enc2)
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes(&self, value: &Self::Type) -> std::io::Result<Vec<u8>> {
        let mut result = self.len.to_bytes(&value.len())?;
        result.extend(str_to_bytes(value, &self.enc1, &self.enc2)?);
        Ok(result)
    }
}


#[pymethods]
impl Str {
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

    pub fn __getitem__(&self, encodings: &Bound<PyAny>) -> PyResult<BfpType> {
        if let Ok(enc1) = encodings.extract::<Encoding>() {
            return Ok(BfpType::Str(Str { len: self.len.clone(), enc1, enc2: None }));
        }
        let Ok(tup) = encodings.downcast::<PyTuple>() else {
            return Err(PyTypeError::new_err("Only encodings may be specified as arguments to string types"))
        };
        if tup.len() != 2 {
            return Err(PyTypeError::new_err("Only a maximum of two encodings may be provided. Help: Check for trailing commas"))
        }
        let (enc1, enc2) = unsafe { (tup.get_item_unchecked(0), tup.get_item_unchecked(1)) };
        let (enc1, enc2) = (enc1.extract()?, enc2.extract()?);
        
        Ok(BfpType::Str(Str { len: self.len.clone(), enc1, enc2: Some(enc2) }))
    }
}
