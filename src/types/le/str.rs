use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple, PyType};
use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::le::encoding::Encoding;
use crate::types::le::size::Size;
use crate::types::le::str_array::StrArray;
use crate::types::le::utils::{str_from_bytes, str_to_bytes};
use crate::types::parseable::Parseable;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.types.le", name = "Str")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Str {
    pub len_type: Size,
    enc1: Encoding,
    enc2: Option<Encoding>,
}

impl Str {
    pub fn len_size(len_type: Size) -> Self {
        Self {
            len_type,
            enc1: Encoding::UTF8,
            enc2: Some(Encoding::LATIN1),
        }
    }
    
    pub fn from_arr(arr: &StrArray) -> Self {
        Self {
            len_type: arr.str_len_type.clone(),
            enc1: arr.enc1.clone(),
            enc2: arr.enc2.clone(),
        }
    }
}

impl Parseable for Str {
    type Type = String;

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream(&self, stream: &mut ByteStream, _ver: &Version) -> PyResult<Self::Type> {
        let len = self.len_type.from_stream(stream, _ver)?;
        let bytes = stream.get(len)?;
        str_from_bytes(bytes, &self.enc1, &self.enc2)
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()> {
        let num_len_bytes = self.len_type.num_bytes();
        
        let start = buffer.len();
        buffer.resize(buffer.len() + num_len_bytes, 0);
        
        let content_start = buffer.len();
        str_to_bytes(value, &self.enc1, &self.enc2, buffer)?;
        
        let len_bytes = self.len_type.to_bytes_array(buffer.len() - content_start)?;
        buffer[start..content_start].copy_from_slice(&len_bytes[..num_len_bytes]);
        Ok(())
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
            return Ok(BfpType::Str(Str { len_type: self.len_type.clone(), enc1, enc2: None }));
        }
        let Ok(tup) = encodings.downcast::<PyTuple>() else {
            return Err(PyTypeError::new_err("Only encodings may be specified as arguments to string types"))
        };
        if tup.len() != 2 {
            return Err(PyTypeError::new_err("Only a maximum of two encodings may be provided. Help: Check for trailing commas"))
        }
        let (enc1, enc2) = unsafe { (tup.get_item_unchecked(0), tup.get_item_unchecked(1)) };
        let (enc1, enc2) = (enc1.extract()?, enc2.extract()?);
        
        Ok(BfpType::Str(Str { len_type: self.len_type.clone(), enc1, enc2: Some(enc2) }))
    }

    #[classmethod]
    pub fn __class_getitem__(_cls: &Bound<PyType>, len: usize) -> PyResult<BfpType> {
        Ok(BfpType::Str(Str::len_size(Size::Fixed(len))))
    }
}
