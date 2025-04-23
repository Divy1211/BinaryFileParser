use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList};

use crate::types::bfp_list::BfpList;
use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::le::array::Array;
use crate::types::le::size::Size;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.types.le", name = "StackedArray")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackedArrayBuilder {
    pub len_type: Size,
    pub ls_len_type: Size,
}

impl StackedArrayBuilder {
    pub fn new(len_type: Size) -> Self {
        Self {
            ls_len_type: len_type.clone(),
            len_type,
        }
    }
    
    pub fn fixed(slf: PyRef<Self>, len: usize) -> Self {
        Self { 
            len_type: Size::Fixed(len),
            ls_len_type: slf.ls_len_type.clone(),
        }
    }
}

#[pymethods]
impl StackedArrayBuilder {
    pub fn __getitem__<'py>(slf: PyRef<'py, Self>, bfp_type: &Bound<PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(len) = bfp_type.extract::<usize>() {
            return Ok(Bound::new(slf.py(), StackedArrayBuilder::fixed(slf, len))?.into_any())
        };
        let bfp_type = BfpType::from_py_any(bfp_type)?;
        Ok(Bound::new(slf.py(), BfpType::StackedArray(StackedArray::new(slf, bfp_type)))?.into_any())
    }
}


#[pyclass(module = "bfp_rs.types.le", name = "StackedArray")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackedArray {
    pub len_type: Size,
    pub ls_len_type: Size,
    pub data_type: Box<BfpType>,
}

impl StackedArray {
    pub fn new(arr: PyRef<StackedArrayBuilder>, bfp_type: BfpType) -> Self {
        Self {
            len_type: arr.len_type.clone(),
            ls_len_type: arr.ls_len_type.clone(),
            data_type: Box::new(bfp_type)
        }
    }
    pub fn get_bfp_ls(&self, ls: &Bound<PyAny>) -> PyResult<BfpList> {
        Ok(match ls.extract::<BfpList>() {
            Ok(ls) => {
                let inner = ls.inner();
                let BfpType::Array(type_) = &inner.data_type else {
                    return Err(PyTypeError::new_err(format!(
                        "List type mismatch, assigning list[{}] to list[list[{}]]", inner.data_type.py_name(), self.data_type.py_name()
                    )));
                };
                if self.data_type != type_.data_type {
                    return Err(PyTypeError::new_err(format!(
                        "List type mismatch, assigning list[list[{}]] to list[list[{}]]", type_.data_type.py_name(), self.data_type.py_name()
                    )))
                };
                drop(inner);
                ls
            },
            Err(_) => {
                let array_type = Array::from_stacked(self);
                let ls = ls.downcast::<PyList>()?.iter()
                    .map(|value| array_type.get_bfp_ls(&value).map(ParseableType::from))
                    .collect::<PyResult<Vec<_>>>()?;
                BfpList::new(ls, BfpType::Array(array_type))
            }
        })
    }
}

impl Parseable for StackedArray {
    type Type = BfpList;

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> PyResult<Self::Type> {
        let len = self.len_type.from_stream(stream, ver)?;
        let mut lens = Vec::with_capacity(len);
        for _ in 0..len {
            lens.push(self.ls_len_type.from_stream(stream, ver)?);
        }
        let mut lss: Vec<ParseableType> = Vec::with_capacity(len);
        for len in lens {
            let mut items = Vec::with_capacity(len);
            for _ in 0..len {
                items.push(self.data_type.from_stream(stream, ver)?);
            }
            lss.push(BfpList::new(items, *self.data_type.clone()).into());
        }
        Ok(BfpList::new(lss, BfpType::Array(Array::from_stacked(self))))
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()> {
        let inner = value.inner();
        self.len_type.to_bytes_in(&inner.data.len(), buffer)?;

        let num_len_bytes = self.ls_len_type.num_bytes();

        let mut start = buffer.len();
        buffer.resize(buffer.len() + inner.data.len() * num_len_bytes, 0);

        for ls in inner.data.iter() {
            let ParseableType::Array(ls) = ls else {
                unreachable!("All code paths to this fn go through StackedArray::get_bfp_ls")
            };
            let inner = ls.inner();
            let len_bytes = self.ls_len_type.to_bytes_array(inner.data.len())?;
            buffer[start..start+num_len_bytes].copy_from_slice(&len_bytes[..num_len_bytes]);
            start += num_len_bytes;

            for item in inner.data.iter() {
                self.data_type.to_bytes_in(item, buffer)?;
            }
        }

        Ok(())
    }
}


#[pymethods]
impl StackedArray {
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
}
