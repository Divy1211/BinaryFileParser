use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList, PyType};
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_list::BfpList;
use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::context::Context;
use crate::types::le::option::OptionType;
use crate::types::le::size::Size;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::r#struct::Struct;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.types.le", name = "StackedAttrArray")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackedAttrArrayBuilder {
    pub len_type: Size,
}

impl StackedAttrArrayBuilder {
    pub fn new(len_type: Size) -> Self {
        Self { len_type }
    }
}

#[pymethods]
impl StackedAttrArrayBuilder {
    pub fn __getitem__(slf: PyRef<Self>, bfp_type: &Bound<PyAny>) -> PyResult<BfpType> {
        let bfp_type = BfpType::from_py_any(bfp_type)?;
        match &bfp_type {
            BfpType::Option(_) => {},
            BfpType::Struct(_) => {},
            _ => {
                return Err(PyTypeError::new_err("Only a BaseStruct or Option type can be used with StackedAttrArrayX"))
            }
        }
        Ok(BfpType::StackedAttrArray(StackedAttrArray::new(slf.len_type.clone(), bfp_type)))
    }
}


#[pyclass(module = "bfp_rs.types.le", name = "StackedAttrArray")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackedAttrArray {
    pub len_type: Size,
    pub data_type: Box<BfpType>,
}

impl StackedAttrArray {
    pub fn new(len_type: Size, bfp_type: BfpType) -> Self {
        Self { len_type, data_type: Box::new(bfp_type) }
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
                let ls = ls.cast::<PyList>()?.iter()
                    .map(|value| self.data_type.to_parseable(&value))
                    .collect::<PyResult<Vec<_>>>()?;
                BfpList::new(ls, *self.data_type.clone())
            }
        })
    }
}

impl StackedAttrArray {
    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream_option(&self, stream: &mut ByteStream, ver: &Version, type_: &OptionType, ctx: &mut Context) -> PyResult<<Self as Parseable>::Type> {
        let len = self.len_type.from_stream_ctx(stream, ver, ctx)?;
        let mut exist_flags = Vec::with_capacity(len);
        let mut items = Vec::with_capacity(len);
        for _ in 0..len {
            exist_flags.push(type_.len_type.from_stream_ctx(stream, ver, ctx)?);
        }
        for exists in exist_flags {
            if exists != 0 {
                items.push(Some(Box::new(type_.data_type.from_stream_ctx(stream, ver, ctx)?)).into());
            } else {
                items.push(None.into());
            }
        }

        Ok(BfpList::new(items, BfpType::Option(type_.clone())))
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes_option(&self, value: &<Self as Parseable>::Type, type_: &OptionType, buffer: &mut Vec<u8>) -> PyResult<()> {
        let inner = value.inner();
        self.len_type.to_bytes_in(&inner.data.len(), buffer)?;
        
        let num_exist_bytes = type_.len_type.num_bytes();
        
        let mut start = buffer.len();
        buffer.resize(buffer.len() + inner.data.len() * num_exist_bytes, 0);
        
        for item in inner.data.iter() {
            let ParseableType::Option(item) = item else {
                unreachable!("All code paths to this option fn go through StackedAttrArray::get_bfp_ls")
            };

            let mut exists = 0;
            if let Some(item) = item {
                exists = 1;
                type_.data_type.to_bytes_in(item.as_ref(), buffer)?;
            }
            
            let exist_bytes = type_.len_type.to_bytes_array(exists)?;
            buffer[start..start+ num_exist_bytes].copy_from_slice(&exist_bytes[..num_exist_bytes]);
            start += num_exist_bytes;
        }
        Ok(())
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream_struct(&self, stream: &mut ByteStream, ver: &Version, type_: &Struct, ctx: &mut Context) -> PyResult<<Self as Parseable>::Type> {
        let retrievers = type_.retrievers();
        
        let len = self.len_type.from_stream_ctx(stream, ver, ctx)?;
        let mut data_lss = Vec::with_capacity(len);
        for _ in 0..len {
            data_lss.push(Vec::with_capacity(retrievers.len()));
        }
        
        for retriever in retrievers.iter() {
            if !retriever.supported(ver) {
                for ls in data_lss.iter_mut().take(len) {
                    ls.push(None);
                }
                continue;
            }
            for ls in data_lss.iter_mut().take(len) {
                ls.push(Some(retriever.from_stream_ctx(stream, ver, ctx)?));
            }
        }
        
        let structs = data_lss.into_iter().map(|data| {
            ParseableType::Struct {
                val: BaseStruct::new(ver.clone(), data, vec![None; retrievers.len()]),
                struct_: type_.clone(),
            }
        }).collect();
        
        Ok(BfpList::new(structs, BfpType::Struct(type_.clone())))
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes_struct(&self, value: &<Self as Parseable>::Type, type_: &Struct, buffer: &mut Vec<u8>) -> PyResult<()> {
        let retrievers = type_.retrievers();
        let inner = value.inner();
        
        self.len_type.to_bytes_in(&inner.data.len(), buffer)?;
        if inner.data.is_empty() {
            return Ok(());
        }

        let inners = inner.data.iter().map(|value| {
            match value {
                ParseableType::Struct { val, .. } => val.inner(),
                _ => unreachable!("All code paths to this struct fn go through StackedAttrArray::get_bfp_ls")
            }
        }).collect::<Vec<_>>();
        let ver = inners[0].ver.clone();
        
        for retriever in retrievers.iter() {
            if !retriever.supported(&ver) {
                continue;
            }
            for inner in &inners {
                retriever.data_type.to_bytes_in(inner.data[retriever.idx].as_ref().expect("supported check done above"), buffer)?
            }
        }
        
        Ok(())
    }
}

impl Parseable for StackedAttrArray {
    type Type = BfpList;
    
    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream_ctx(&self, stream: &mut ByteStream, ver: &Version, ctx: &mut Context) -> PyResult<Self::Type> {
        match self.data_type.as_ref() {
            BfpType::Option(type_) => { self.from_stream_option(stream, ver, type_, ctx) }
            BfpType::Struct(type_) => { self.from_stream_struct(stream, ver, type_, ctx) }
            _ => unreachable!("User instances of StackedAttrArray type can only be made via builder's __getitem__")
        }
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()> {
        match self.data_type.as_ref() {
            BfpType::Option(type_) => { self.to_bytes_option(value, type_, buffer) }
            BfpType::Struct(type_) => { self.to_bytes_struct(value, type_, buffer) }
            _ => unreachable!("User instances of StackedAttrArray type can only be made via builder's __getitem__")
        }
    }
}


#[pymethods]
impl StackedAttrArray {
    #[pyo3(name = "to_bytes")]
    fn to_bytes_py<'py>(slf: PyRef<'py, Self>, value: &Bound<PyAny>) -> PyResult<Bound<'py, PyBytes>> {
        let bytes = slf.to_bytes(&slf.get_bfp_ls(value)?)?;
        Ok(PyBytes::new(slf.py(), &bytes))
    }

    #[pyo3(name = "from_stream", signature = (stream, ver = Version::new(vec![0,])))]
    fn from_stream_py<'py>(slf: PyRef<'py, Self>, stream: &mut ByteStream, ver: Version) -> PyResult<Bound<'py, PyAny>> {
        let value: ParseableType = slf.from_stream(stream, &ver)?.into();
        value.to_bound(slf.py())
    }

    #[pyo3(name = "from_file")]
    fn from_file_py<'py>(slf: PyRef<'py, Self>, filepath: &str) -> PyResult<Bound<'py, PyAny>> {
        let value: ParseableType = slf.from_file(filepath)?.into();
        value.to_bound(slf.py())
    }
    #[pyo3(name = "from_bytes", signature = (bytes, ver = Version::new(vec![0,])))]
    fn from_bytes_py<'py>(slf: PyRef<'py, Self>, bytes: &[u8], ver: Version) -> PyResult<Bound<'py, PyAny>> {
        let value: ParseableType = slf.from_bytes(bytes, &ver)?.into();
        value.to_bound(slf.py())
    }
    #[pyo3(name = "to_file")]
    fn to_file_py(slf: PyRef<Self>, filepath: &str, value: &Bound<PyAny>) -> PyResult<()> {
        slf.to_file(filepath, &slf.get_bfp_ls(value)?)
    }
    
    #[classmethod]
    fn __class_getitem__(_cls: &Bound<PyType>, len: usize) -> PyResult<StackedAttrArrayBuilder> {
        Ok(StackedAttrArrayBuilder { len_type: Size::Fixed(len) })
    }
}
