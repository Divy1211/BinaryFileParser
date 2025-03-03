use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList, PyType};
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_list::BfpList;
use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
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
                if *self.data_type != ls.data_type {
                    return Err(PyTypeError::new_err(format!(
                        "List type mismatch, assigning list[{}] to list[{}]", ls.data_type.py_name(), self.data_type.py_name()
                    )))
                };
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

impl StackedAttrArray {
    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream_option(&self, stream: &mut ByteStream, ver: &Version, type_: &OptionType) -> std::io::Result<<Self as Parseable>::Type> {
        let len = self.len_type.from_stream(stream, ver)?;
        let mut exist_flags = Vec::with_capacity(len);
        let mut items = Vec::with_capacity(len);
        for _ in 0..len {
            exist_flags.push(type_.len_type.from_stream(stream, ver)?);
        }
        for exists in exist_flags {
            if exists != 0 {
                items.push(Some(Box::new(type_.data_type.from_stream(stream, ver)?)).into());
            } else {
                items.push(None.into());
            }
        }

        Ok(BfpList::new(items, BfpType::Option(type_.clone())))
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes_option(&self, value: &<Self as Parseable>::Type, type_: &OptionType) -> std::io::Result<Vec<u8>> {
        let ls = value.ls.read().expect("GIL bound read");
        
        let mut bytes = self.len_type.to_bytes(&ls.len())?;
        
        let mut exist_bytes = Vec::with_capacity(ls.len());
        let mut ls_bytes = Vec::with_capacity(ls.len());
        for item in ls.iter() {
            let ParseableType::Option(item) = item else { unreachable!("All code paths to this option fn go through StackedAttrArray::get_bfp_ls") };
            match item.as_ref() {
                None => { exist_bytes.append(&mut type_.len_type.to_bytes(&0)?) }
                Some(item) => {
                    exist_bytes.append(&mut type_.len_type.to_bytes(&1)?);
                    ls_bytes.append(&mut type_.data_type.to_bytes(item.as_ref())?);
                }
            }
        }
        bytes.append(&mut exist_bytes);
        bytes.append(&mut ls_bytes);
        Ok(bytes)
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream_struct(&self, stream: &mut ByteStream, ver: &Version, type_: &Struct) -> std::io::Result<<Self as Parseable>::Type> {
        let retrievers = type_.retrievers.read().expect("GIL bound read");
        
        let len = self.len_type.from_stream(stream, ver)?;
        let mut data_lss = Vec::with_capacity(len);
        for _ in 0..len {
            data_lss.push(Vec::with_capacity(retrievers.len()));
        }
        
        for retriever in retrievers.iter() {
            if !retriever.supported(ver) {
                for i in 0..len {
                    data_lss[i].push(None);
                }
                continue;
            }
            for i in 0..len {
                data_lss[i].push(Some(retriever.from_stream(stream, ver)?));
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
    fn to_bytes_struct(&self, value: &<Self as Parseable>::Type, type_: &Struct) -> std::io::Result<Vec<u8>> {
        let retrievers = type_.retrievers.read().expect("GIL bound read");
        let structs = value.ls.read().expect("GIL bound read");
        
        let mut bytes = self.len_type.to_bytes(&structs.len())?;
        if structs.len() == 0 {
            return Ok(bytes);
        }
        let mut ver = None;
        let data_lss = structs.iter().map(|value| {
            match value {
                ParseableType::Struct { val, .. } => {
                    ver = Some(val.ver.clone());
                    val.data.read().expect("GIL bound read")
                },
                _ => unreachable!("All code paths to this struct fn go through StackedAttrArray::get_bfp_ls")
            }
        }).collect::<Vec<_>>();
        let ver = ver.expect("At least one item in ls");
        
        for (i, retriever) in retrievers.iter().enumerate() {
            if !retriever.supported(&ver) {
                continue;
            }
            for data in data_lss.iter() {
                bytes.append(&mut retriever.data_type.to_bytes(data[i].as_ref().expect("supported check done above"))?)
            }
        }
        
        Ok(bytes)
    }
}

impl Parseable for StackedAttrArray {
    type Type = BfpList;
    
    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> std::io::Result<Self::Type> {
        match self.data_type.as_ref() {
            BfpType::Option(type_) => { self.from_stream_option(stream, ver, type_) }
            BfpType::Struct(type_) => { self.from_stream_struct(stream, ver, type_) }
            _ => unreachable!("User instances of StackedAttrArray type can only be made via builder's __getitem__")
        }
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes(&self, value: &Self::Type) -> std::io::Result<Vec<u8>> {
        match self.data_type.as_ref() {
            BfpType::Option(type_) => { self.to_bytes_option(value, type_) }
            BfpType::Struct(type_) => { self.to_bytes_struct(value, type_) }
            _ => unreachable!("User instances of StackedAttrArray type can only be made via builder's __getitem__")
        }
    }
}


#[pymethods]
impl StackedAttrArray {
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
    fn __class_getitem__(_cls: &Bound<PyType>, len: usize) -> PyResult<StackedAttrArrayBuilder> {
        Ok(StackedAttrArrayBuilder { len_type: Size::Fixed(len) })
    }
}
