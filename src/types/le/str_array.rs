use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};

use crate::types::bfp_list::BfpList;
use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::le::encoding::Encoding;
use crate::types::le::size::Size;
use crate::types::le::str::Str;
use crate::types::le::utils::{str_from_bytes, str_to_bytes};
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.types.le", name = "StrArray")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrArray {
    pub len_type: Size,
    pub str_len_type: Size,
    pub enc1: Encoding,
    pub enc2: Option<Encoding>,
}

impl StrArray {
    pub fn len_size(len_type: Size) -> Self {
        Self {
            str_len_type: len_type.clone(),
            len_type,
            enc1: Encoding::UTF8,
            enc2: Some(Encoding::LATIN1),
        }
    }

    pub fn len_sizes(len_type: Size, str_len_type: Size) -> Self {
        Self {
            str_len_type,
            len_type,
            enc1: Encoding::UTF8,
            enc2: Some(Encoding::LATIN1),
        }
    }
    
    pub fn get_bfp_ls(&self, ls: &Bound<PyAny>) -> PyResult<BfpList> {
        Ok(match ls.extract::<BfpList>() {
            Ok(ls) => {
                let inner = ls.inner();
                let BfpType::Str(_) = inner.data_type else {
                    return Err(PyTypeError::new_err(format!(
                        "List type mismatch, assigning list[{}] to list[str]", inner.data_type.py_name()
                    )))
                };
                drop(inner);
                ls
            },
            Err(_) => {
                let ls = ls.extract::<Vec<String>>()?;
                let ls = ls.into_iter().map(ParseableType::from).collect();
                BfpList::new(ls, BfpType::Str(Str::from_arr(self)))
            }
        })
    }
}

impl Parseable for StrArray {
    type Type = BfpList;

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream(&self, stream: &mut ByteStream, _ver: &Version) -> PyResult<Self::Type> {
        let len = self.len_type.from_stream(stream, _ver)?;
        
        let mut lens = Vec::with_capacity(len);
        let mut ls = Vec::with_capacity(len);
        for _ in 0..len {
            lens.push(self.str_len_type.from_stream(stream, _ver)?);
        }
        
        for len in lens {
            let bytes = stream.get(len)?;
            ls.push(str_from_bytes(bytes, &self.enc1, &self.enc2)?.into());
        }
        Ok(BfpList::new(ls, BfpType::Str(Str::from_arr(self))))
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()> {
        let inner = value.inner();
        let data = inner.data.iter()
            .map(String::try_from)
            .map(|result| result.expect("All code paths to this fn go through StrArray::get_bfp_ls"));

        self.len_type.to_bytes_in(&inner.data.len(), buffer)?;
        
        let num_len_bytes = self.str_len_type.num_bytes();
        
        let mut start = buffer.len();
        buffer.resize(buffer.len() + inner.data.len() * num_len_bytes, 0);
        
        for string in data {
            let content_start = buffer.len();
            str_to_bytes(&string, &self.enc1, &self.enc2, buffer)?;
            
            let len_bytes = self.str_len_type.to_bytes_array(buffer.len() - content_start)?;
            
            buffer[start..start+num_len_bytes].copy_from_slice(&len_bytes[..num_len_bytes]);
            start += num_len_bytes;
        }
        Ok(())
    }
}


#[pymethods]
impl StrArray {
    #[pyo3(name = "to_bytes")]
    fn to_bytes_py<'py>(slf: PyRef<'py, Self>, value: &Bound<PyAny>) -> PyResult<Bound<'py, PyBytes>> {
        let bytes = slf.to_bytes(&slf.get_bfp_ls(value)?)?;
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
    fn to_file_py(slf: PyRef<Self>, filepath: &str, value: &Bound<PyAny>) -> PyResult<()> {
        Ok(slf.to_file(filepath, &slf.get_bfp_ls(value)?)?)
    }

    pub fn __getitem__(&self, len_or_encodings: &Bound<PyAny>) -> PyResult<BfpType> {
        if let Ok(len) = len_or_encodings.extract::<usize>() {
            return Ok(BfpType::StrArray(StrArray {
                len_type: Size::Fixed(len),
                str_len_type: self.str_len_type.clone(),
                enc1: self.enc1.clone(),
                enc2: self.enc2.clone()
            }))
        }
        if let Ok(enc1) = len_or_encodings.extract::<Encoding>() {
            return Ok(BfpType::StrArray(StrArray {
                len_type: self.len_type.clone(),
                str_len_type: self.str_len_type.clone(),
                enc1,
                enc2: None
            }));
        }
        let Ok(tup) = len_or_encodings.downcast::<PyTuple>() else {
            return Err(PyTypeError::new_err("Only encodings may be specified as arguments to string types"))
        };
        if tup.len() != 2 {
            return Err(PyTypeError::new_err("Only a maximum of two encodings may be provided. Help: Check for trailing commas"))
        }
        let (enc1, enc2) = unsafe { (tup.get_item_unchecked(0), tup.get_item_unchecked(1)) };
        let (enc1, enc2) = (enc1.extract()?, enc2.extract()?);

        Ok(BfpType::StrArray(StrArray {
            len_type: self.len_type.clone(),
            str_len_type: self.str_len_type.clone(),
            enc1,
            enc2: Some(enc2)
        }))
    }
}
