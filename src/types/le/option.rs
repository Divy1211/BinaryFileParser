use pyo3::prelude::*;
use pyo3::types::{PyBytes};

use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::le::size::Size;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.types.le", name = "Option")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionBuilder {
    pub len_type: Size,
}

impl OptionBuilder {
    pub fn new(len_type: Size) -> Self {
        Self { len_type }
    }
}

#[pymethods]
impl OptionBuilder {
    pub fn __getitem__(slf: PyRef<Self>, bfp_type: &Bound<PyAny>) -> PyResult<BfpType> {
        let bfp_type = BfpType::from_py_any(bfp_type)?;
        Ok(BfpType::Option(OptionType::new(slf.len_type.clone(), bfp_type)))
    }
}


#[pyclass(module = "bfp_rs.types.le", name = "OptionType")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionType {
    pub len_type: Size,
    pub data_type: Box<BfpType>,
}

impl OptionType {
    pub fn new(len_type: Size, bfp_type: BfpType) -> Self {
        Self { len_type, data_type: Box::new(bfp_type) }
    }
    
    pub fn get_option(&self, value: &Bound<PyAny>) -> PyResult<Option<Box<ParseableType>>> {
        if value.is_none() {
            Ok(None)
        } else {
            Ok(Some(Box::new(self.data_type.to_parseable(value)?)))
        }
    }
}

impl Parseable for OptionType {
    type Type = Option<Box<ParseableType>>;

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> PyResult<Self::Type> {
        let exists = self.len_type.from_stream(stream, ver)?;
        if exists == 0 {
            return Ok(None);
        }
        Ok(Some(Box::new(self.data_type.from_stream(stream, ver)?)))
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()> {
        let Some(value) = value else {
            return self.len_type.to_bytes_in(&0, buffer);
        };
        self.len_type.to_bytes_in(&1, buffer)?;
        self.data_type.to_bytes_in(&value, buffer)?;
        Ok(())
    }
}


#[pymethods]
impl OptionType {
    #[pyo3(name = "to_bytes")]
    fn to_bytes_py<'py>(slf: PyRef<'py, Self>, value: &Bound<PyAny>) -> PyResult<Bound<'py, PyBytes>> {
        let bytes = slf.to_bytes(&slf.get_option(value)?)?;
        Ok(PyBytes::new_bound(slf.py(), &bytes))
    }

    #[pyo3(name = "from_stream", signature = (stream, ver = Version::new(vec![0,])))]
    fn from_stream_py<'py>(slf: PyRef<'py, Self>, stream: &mut ByteStream, ver: Version) -> PyResult<Bound<'py, PyAny>> {
        let value: ParseableType = slf.from_stream(stream, &ver)?.into(); 
        Ok(value.to_bound(slf.py()))
    }

    #[pyo3(name = "from_file")]
    fn from_file_py<'py>(slf: PyRef<'py, Self>, filepath: &str) -> PyResult<Bound<'py, PyAny>> {
        let value: ParseableType = slf.from_file(filepath)?.into();
        Ok(value.to_bound(slf.py()))
    }
    #[pyo3(name = "from_bytes", signature = (bytes, ver = Version::new(vec![0,])))]
    fn from_bytes_py<'py>(slf: PyRef<'py, Self>, bytes: &[u8], ver: Version) -> PyResult<Bound<'py, PyAny>> {
        let value: ParseableType = slf.from_bytes(bytes, &ver)?.into();
        Ok(value.to_bound(slf.py()))
    }
    #[pyo3(name = "to_file")]
    fn to_file_py(slf: PyRef<Self>, filepath: &str, value: &Bound<PyAny>) -> PyResult<()> {
        Ok(slf.to_file(filepath, &slf.get_option(value)?)?)
    }
}
