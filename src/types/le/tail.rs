use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList, PyType};

use crate::types::bfp_list::BfpList;
use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;


#[pyclass(module = "bfp_rs.types.le", name = "Tail")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tail {
    pub data_type: Box<BfpType>,
}

impl Tail {
    pub fn new(bfp_type: BfpType) -> Self {
        Self { data_type: Box::new(bfp_type) }
    }

    pub fn get_bfp_ls(&self, ls: &Bound<PyAny>) -> PyResult<BfpList> {
        Ok(match ls.extract::<BfpList>() {
            Ok(ls) => {
                let inner = ls.inner();
                if *self.data_type != inner.data_type {
                    return Err(PyTypeError::new_err(format!(
                        "List type mismatch, assigning list[{}] to list[{}]", inner.data_type.py_name(), self.data_type.py_name()
                    )))
                };
                drop(inner);
                ls
            },
            Err(_) => {
                let ls = ls.downcast::<PyList>()?.iter()
                    .map(|value| self.data_type.to_parseable(&value))
                    .collect::<PyResult<Vec<_>>>()?;
                BfpList::new(ls, *self.data_type.clone())
            }
        })
    }
}

impl Parseable for Tail {
    type Type = BfpList;

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> PyResult<Self::Type> {
        let mut ls = Vec::new();

        while !stream.is_empty() {
            ls.push(self.data_type.from_stream(stream, ver)?);
        }

        Ok(BfpList::new(ls, *self.data_type.clone()))
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()> {
        let inner = value.inner();
        for item in inner.data.iter() {
            self.data_type.to_bytes_in(item, buffer)?;
        }
        Ok(())
    }
}


#[pymethods]
impl Tail {
    #[pyo3(name = "to_bytes")]
    fn to_bytes_py<'py>(slf: PyRef<'py, Self>, value: &Bound<PyAny>) -> PyResult<Bound<'py, PyBytes>> {
        let bytes = slf.to_bytes(&slf.get_bfp_ls(value)?)?;
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
        Ok(slf.to_file(filepath, &slf.get_bfp_ls(value)?)?)
    }

    #[classmethod]
    pub fn __class_getitem__(_cls: &Bound<PyType>, bfp_type: &Bound<PyAny>) -> PyResult<BfpType> {
        let bfp_type = BfpType::from_py_any(bfp_type)?;
        Ok(BfpType::Tail(Tail::new(bfp_type)))
    }
}
