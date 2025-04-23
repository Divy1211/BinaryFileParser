use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::{pyclass, pymethods, Bound, PyAny, PyResult};
use pyo3::types::{PyBytes, PyType};
use crate::types::base_struct::BaseStruct;
use crate::types::byte_stream::ByteStream;
use crate::types::le::bool::{Bool128, Bool16, Bool32, Bool64, Bool8};
use crate::types::le::bytes::Bytes;
use crate::types::le::float::{Float32, Float64};
use crate::types::le::int::{Int128, Int16, Int32, Int64, Int8, UInt128, UInt16, UInt32, UInt64, UInt8};
use crate::types::le::nt_str::NtStr;
use crate::types::le::option::OptionType;
use crate::types::le::size::Size;
use crate::types::le::str::Str;
use crate::types::le::str_array::StrArray;
use crate::types::le::array::Array;
use crate::types::le::stacked_array::StackedArray;
use crate::types::le::stacked_attr_array::StackedAttrArray;
use crate::types::le::tail::Tail;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::r#struct::Struct;
use crate::types::struct_builder::StructBuilder;
use crate::types::version::Version;

// todo: change to a structural enum
#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BfpType {
    UInt8(UInt8),
    UInt16(UInt16),
    UInt32(UInt32),
    UInt64(UInt64),
    UInt128(UInt128),
    
    Int8(Int8),
    Int16(Int16),
    Int32(Int32),
    Int64(Int64),
    Int128(Int128),

    Float32(Float32),
    Float64(Float64),

    Bool8(Bool8),
    Bool16(Bool16),
    Bool32(Bool32),
    Bool64(Bool64),
    Bool128(Bool128),

    Bytes(Bytes),

    Str(Str),
    NTStr(NtStr),
    
    StrArray(StrArray),
    
    Option(OptionType),
    
    Array(Array),
    StackedArray(StackedArray),
    StackedAttrArray(StackedAttrArray),

    Tail(Tail),
    
    Struct(Struct),
}

impl BfpType {
    pub fn get_contained_type(&self, name: &str) -> PyResult<BfpType> {
        Ok(match self {
            BfpType::StrArray(arr)         => BfpType::Str(Str::from_arr(arr)),
            BfpType::Array(arr)            => arr.data_type.as_ref().clone(),
            BfpType::StackedArray(arr)     => arr.data_type.as_ref().clone(),
            BfpType::StackedAttrArray(arr) => arr.data_type.as_ref().clone(),
            BfpType::Tail(arr)             => arr.data_type.as_ref().clone(),
            _ => {
                return Err(PyTypeError::new_err(format!(
                    "Cannot index a type '{}', attempting to index '{}'", self.py_name(), name
                )))
            }
        })
    }
    
    pub fn from_py_any(value: &Bound<PyAny>) -> PyResult<BfpType> {
        Ok(match value.extract::<BfpType>() {
            Ok(type_) => type_,
            Err(_) => {
                let cls = value.downcast::<PyType>()?;
                if !cls.is_subclass_of::<BaseStruct>()? {
                    return Err(PyTypeError::new_err(
                        "Cannot create a BfpType from a class that does not subclass BaseStruct"
                    ))
                }
                BfpType::Struct(StructBuilder::get_struct(cls)?)
            },
        })
    }
    
    pub fn is_ord(&self) -> bool {
        match self {
            BfpType::Struct(_) => false,
            _ => true,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            BfpType::Bool8(_)   => true,
            BfpType::Bool16(_)  => true,
            BfpType::Bool32(_)  => true,
            BfpType::Bool64(_)  => true,
            BfpType::Bool128(_) => true,
            _ => false,
        }
    }

    pub fn py_name(&self) -> String {
        match self {
            BfpType::UInt8(_)                 => "int".into(),
            BfpType::UInt16(_)                => "int".into(),
            BfpType::UInt32(_)                => "int".into(),
            BfpType::UInt64(_)                => "int".into(),
            BfpType::UInt128(_)               => "int".into(),

            BfpType::Int8(_)                  => "int".into(),
            BfpType::Int16(_)                 => "int".into(),
            BfpType::Int32(_)                 => "int".into(),
            BfpType::Int64(_)                 => "int".into(),
            BfpType::Int128(_)                => "int".into(),

            BfpType::Float32(_)               => "float".into(),
            BfpType::Float64(_)               => "float".into(),

            BfpType::Bool8(_)                 => "bool".into(),
            BfpType::Bool16(_)                => "bool".into(),
            BfpType::Bool32(_)                => "bool".into(),
            BfpType::Bool64(_)                => "bool".into(),
            BfpType::Bool128(_)               => "bool".into(),
            
            BfpType::Bytes(_)                 => "bytes".into(),

            BfpType::Str(_)                   => "str".into(),
            BfpType::NTStr(_)                 => "str".into(),
            
            BfpType::StrArray(_)              => "list[str]".into(),

            BfpType::Option(type_)            => format!("{} | None", type_.data_type.py_name()),

            BfpType::Array(type_)             => format!("list[{}]", type_.data_type.py_name()),
            BfpType::StackedArray(type_)      => format!("list[list[{}]", type_.data_type.py_name()),
            BfpType::StackedAttrArray(type_)  => format!("list[{}]", type_.data_type.py_name()),
            BfpType::Tail(type_)              => format!("list[{}]", type_.data_type.py_name()),
            
            BfpType::Struct(struct_)          => struct_.fully_qualified_name(),
        }
    }

    pub fn to_parseable_from_int(&self, value: i128) -> Option<ParseableType> {
        match self {
            BfpType::UInt8(_)   => { Some(ParseableType::UInt8(value as u8)) },
            BfpType::UInt16(_)  => { Some(ParseableType::UInt16(value as u16)) },
            BfpType::UInt32(_)  => { Some(ParseableType::UInt32(value as u32)) },
            BfpType::UInt64(_)  => { Some(ParseableType::UInt64(value as u64)) },
            BfpType::UInt128(_) => { Some(ParseableType::UInt128(value as u128)) },
            
            BfpType::Int8(_)    => { Some(ParseableType::Int8(value as i8)) },
            BfpType::Int16(_)   => { Some(ParseableType::Int16(value as i16)) },
            BfpType::Int32(_)   => { Some(ParseableType::Int32(value as i32)) },
            BfpType::Int64(_)   => { Some(ParseableType::Int64(value as i64)) },
            BfpType::Int128(_)  => { Some(ParseableType::Int128(value)) },
            
            _                   => { None }
        }
    }
    
    /// converts python values to ParseableTypes
    pub fn to_parseable(&self, value: &Bound<'_, PyAny>) -> PyResult<ParseableType> {
        Ok(match self {
            BfpType::UInt8(_)   => value.extract::<u8>()?.into(),
            BfpType::UInt16(_)  => value.extract::<u16>()?.into(),
            BfpType::UInt32(_)  => value.extract::<u32>()?.into(),
            BfpType::UInt64(_)  => value.extract::<u64>()?.into(),
            BfpType::UInt128(_) => value.extract::<u128>()?.into(),

            BfpType::Int8(_)    => value.extract::<i8>()?.into(),
            BfpType::Int16(_)   => value.extract::<i16>()?.into(),
            BfpType::Int32(_)   => value.extract::<i32>()?.into(),
            BfpType::Int64(_)   => value.extract::<i64>()?.into(),
            BfpType::Int128(_)  => value.extract::<i128>()?.into(),

            BfpType::Float32(_) => value.extract::<f32>()?.into(),
            BfpType::Float64(_) => value.extract::<f64>()?.into(),

            BfpType::Bool8(_)   => value.extract::<bool>()?.into(),
            BfpType::Bool16(_)  => value.extract::<bool>()?.into(),
            BfpType::Bool32(_)  => value.extract::<bool>()?.into(),
            BfpType::Bool64(_)  => value.extract::<bool>()?.into(),
            BfpType::Bool128(_) => value.extract::<bool>()?.into(),

            BfpType::Bytes(Bytes { len })    => {
                let bytes = value.extract::<Vec<u8>>()?;
                if bytes.len() != *len {
                    return Err(PyValueError::new_err(format!(
                        "Attempting to set Bytes[{}] to a bytestring of length {}", len, bytes.len(),
                    )));
                }
                bytes.into()
            },

            BfpType::Str(_)         => value.extract::<String>()?.into(),
            BfpType::NTStr(_)       => value.extract::<String>()?.into(),
            BfpType::Option(type_)  => type_.get_option(value)?.into(),
            
            BfpType::StrArray(type_) => {
                let ls = type_.get_bfp_ls(value)?;
                let Size::Fixed(len) = type_.len_type else {
                    return Ok(ls.into());
                };
                if ls.len() != len {
                    return Err(PyValueError::new_err(format!(
                        "Attempting to set StrArrayX[{}] to a list of length {}", len, ls.len(),
                    )));
                }
                ls.into()
            }

            BfpType::Array(type_) => {
                let ls = type_.get_bfp_ls(value)?;
                let Size::Fixed(len) = type_.len_type else {
                    return Ok(ls.into());
                };
                if ls.len() != len {
                    return Err(PyValueError::new_err(format!(
                        "Attempting to set ArrayX[{}] to a list of length {}", len, ls.len(),
                    )));
                }
                ls.into()
            }

            BfpType::StackedArray(type_) => {
                let ls = type_.get_bfp_ls(value)?;
                let Size::Fixed(len) = type_.len_type else {
                    return Ok(ls.into());
                };
                if ls.len() != len {
                    return Err(PyValueError::new_err(format!(
                        "Attempting to set StackedArrayX[{}] to a list of length {}", len, ls.len(),
                    )));
                }
                ls.into()
            }

            BfpType::StackedAttrArray(type_) => {
                let ls = type_.get_bfp_ls(value)?;
                let Size::Fixed(len) = type_.len_type else {
                    return Ok(ls.into());
                };
                if ls.len() != len {
                    return Err(PyValueError::new_err(format!(
                        "Attempting to set StackedAttrArrayX[{}] to a list of length {}", len, ls.len(),
                    )));
                }
                ls.into()
            }

            BfpType::Tail(type_) => {
                let ls = type_.get_bfp_ls(value)?;
                ls.into()
            }
            
            BfpType::Struct(struct_) => {
                let py_type = struct_.py_type(value.py());
                if !value.is_exact_instance(py_type) {
                    return Err(PyTypeError::new_err(
                        format!(
                            "'{}' object cannot be interpreted as a '{}'",
                            value.get_type().fully_qualified_name()?.to_string(),
                            py_type.fully_qualified_name()?.to_string()
                        )
                    ))
                }
                
                ParseableType::Struct { val: value.extract::<BaseStruct>()?, struct_: struct_.clone() }
            }
        })
    }

    pub fn is_option(&self) -> bool {
        match self {
            BfpType::Option(_) => true,
            _ => false,
        }
    }
}

impl Parseable for BfpType {
    type Type = ParseableType;

    fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> PyResult<Self::Type> {
        Ok(match self {
            BfpType::UInt8(val)               => val.from_stream(stream, ver)?.into(),
            BfpType::UInt16(val)              => val.from_stream(stream, ver)?.into(),
            BfpType::UInt32(val)              => val.from_stream(stream, ver)?.into(),
            BfpType::UInt64(val)              => val.from_stream(stream, ver)?.into(),
            BfpType::UInt128(val)             => val.from_stream(stream, ver)?.into(),
            
            BfpType::Int8(val)                => val.from_stream(stream, ver)?.into(),
            BfpType::Int16(val)               => val.from_stream(stream, ver)?.into(),
            BfpType::Int32(val)               => val.from_stream(stream, ver)?.into(),
            BfpType::Int64(val)               => val.from_stream(stream, ver)?.into(),
            BfpType::Int128(val)              => val.from_stream(stream, ver)?.into(),
            
            BfpType::Float32(val)             => val.from_stream(stream, ver)?.into(),
            BfpType::Float64(val)             => val.from_stream(stream, ver)?.into(),
            
            BfpType::Bool8(val)               => val.from_stream(stream, ver)?.into(),
            BfpType::Bool16(val)              => val.from_stream(stream, ver)?.into(),
            BfpType::Bool32(val)              => val.from_stream(stream, ver)?.into(),
            BfpType::Bool64(val)              => val.from_stream(stream, ver)?.into(),
            BfpType::Bool128(val)             => val.from_stream(stream, ver)?.into(),

            BfpType::Bytes(val)               => val.from_stream(stream, ver)?.into(),

            BfpType::Str(val)                 => val.from_stream(stream, ver)?.into(),
            BfpType::NTStr(val)               => val.from_stream(stream, ver)?.into(),
            
            BfpType::StrArray(val)            => val.from_stream(stream, ver)?.into(),

            BfpType::Option(val)              => val.from_stream(stream, ver)?.into(),

            BfpType::Array(val)               => val.from_stream(stream, ver)?.into(),
            BfpType::StackedArray(val)        => val.from_stream(stream, ver)?.into(),
            BfpType::StackedAttrArray(val)    => val.from_stream(stream, ver)?.into(),

            BfpType::Tail(val)                => val.from_stream(stream, ver)?.into(),
            
            BfpType::Struct(struct_)          => ParseableType::Struct {
                val: struct_.from_stream(stream, ver)?,
                struct_: struct_.clone()
            },
        })
    }
    
    fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()> {
        match (self, value) {
            (BfpType::UInt8(type_),            ParseableType::UInt8(val))         => type_.to_bytes_in(val, buffer),
            (BfpType::UInt16(type_),           ParseableType::UInt16(val))        => type_.to_bytes_in(val, buffer),
            (BfpType::UInt32(type_),           ParseableType::UInt32(val))        => type_.to_bytes_in(val, buffer),
            (BfpType::UInt64(type_),           ParseableType::UInt64(val))        => type_.to_bytes_in(val, buffer),
            (BfpType::UInt128(type_),          ParseableType::UInt128(val))       => type_.to_bytes_in(val, buffer),

            (BfpType::Int8(type_),             ParseableType::Int8(val))          => type_.to_bytes_in(val, buffer),
            (BfpType::Int16(type_),            ParseableType::Int16(val))         => type_.to_bytes_in(val, buffer),
            (BfpType::Int32(type_),            ParseableType::Int32(val))         => type_.to_bytes_in(val, buffer),
            (BfpType::Int64(type_),            ParseableType::Int64(val))         => type_.to_bytes_in(val, buffer),
            (BfpType::Int128(type_),           ParseableType::Int128(val))        => type_.to_bytes_in(val, buffer),

            (BfpType::Float32(type_),          ParseableType::Float32(val))       => type_.to_bytes_in(val, buffer),
            (BfpType::Float64(type_),          ParseableType::Float64(val))       => type_.to_bytes_in(val, buffer),

            (BfpType::Bool8(type_),            ParseableType::Bool(val))          => type_.to_bytes_in(val, buffer),
            (BfpType::Bool16(type_),           ParseableType::Bool(val))          => type_.to_bytes_in(val, buffer),
            (BfpType::Bool32(type_),           ParseableType::Bool(val))          => type_.to_bytes_in(val, buffer),
            (BfpType::Bool64(type_),           ParseableType::Bool(val))          => type_.to_bytes_in(val, buffer),
            (BfpType::Bool128(type_),          ParseableType::Bool(val))          => type_.to_bytes_in(val, buffer),

            (BfpType::Bytes(type_),            ParseableType::Bytes(val))         => type_.to_bytes_in(val, buffer),

            (BfpType::Str(type_),              ParseableType::Str(val))           => type_.to_bytes_in(val, buffer),
            (BfpType::NTStr(type_),            ParseableType::Str(val))           => type_.to_bytes_in(val, buffer),

            (BfpType::StrArray(type_),         ParseableType::Array(val))         => type_.to_bytes_in(val, buffer),

            (BfpType::Option(type_),           ParseableType::Option(val))        => type_.to_bytes_in(val, buffer),

            (BfpType::Array(type_),            ParseableType::Array(val))         => type_.to_bytes_in(val, buffer),
            (BfpType::StackedArray(type_),     ParseableType::Array(val))         => type_.to_bytes_in(val, buffer),
            (BfpType::StackedAttrArray(type_), ParseableType::Array(val))         => type_.to_bytes_in(val, buffer),
            (BfpType::Tail(type_),             ParseableType::Array(val))         => type_.to_bytes_in(val, buffer),

            (BfpType::Struct(type_),           ParseableType::Struct { val, .. }) => type_.to_bytes_in(val, buffer),

            (type_, val) => {
                unreachable!("BFP Internal Error: Unhandled types {:?} {:?}", type_.py_name(), val)
            }
        }
    }
}

#[pymethods]
impl BfpType {
    #[pyo3(name = "to_bytes")]
    fn to_bytes_py<'py>(slf: PyRef<'py, Self>, value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyBytes>> {
        let bytes = slf.to_bytes(&slf.to_parseable(value)?)?;
        Ok(PyBytes::new_bound(slf.py(), &bytes))
    }

    #[pyo3(name = "from_stream", signature = (stream, ver = Version::new(vec![0,])))]
    fn from_stream_py<'py>(slf: PyRef<'py, Self>, stream: &mut ByteStream, ver: Version) -> PyResult<Bound<'py, PyAny>> {
        Ok(slf.from_stream(stream, &ver)?.to_bound(slf.py()))
    }

    #[pyo3(name = "from_file")]
    fn from_file_py<'py>(slf: PyRef<'py, Self>, filepath: &str) -> PyResult<Bound<'py, PyAny>> {
        Ok(slf.from_file(filepath)?.to_bound(slf.py()))
    }
    #[pyo3(name = "from_bytes", signature = (bytes, ver = Version::new(vec![0,])))]
    fn from_bytes_py<'py>(slf: PyRef<'py, Self>, bytes: &[u8], ver: Version) -> PyResult<Bound<'py, PyAny>> {
        Ok(slf.from_bytes(bytes, &ver)?.to_bound(slf.py()))
    }
    #[pyo3(name = "to_file")]
    fn to_file_py(slf: PyRef<Self>, filepath: &str, value: &Bound<PyAny>) -> PyResult<()> {
        Ok(slf.to_file(filepath, &slf.to_parseable(value)?)?)
    }
}