use crate::types::base_struct::BaseStruct;
use crate::types::byte_stream::ByteStream;
use crate::types::le::bool::{Bool128, Bool16, Bool32, Bool64, Bool8};
use crate::types::le::float::{Float32, Float64};
use crate::types::le::int::{Int128, Int16, Int32, Int64, Int8, UInt128, UInt16, UInt32, UInt64, UInt8};
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::r#struct::Struct;
use crate::types::version::Version;

use pyo3::{pyclass, Bound, PyAny, PyResult};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::{PyAnyMethods, PyTypeMethods};

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

    Struct(Struct),
}

impl BfpType {
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
            BfpType::UInt8(_)   => "int",
            BfpType::UInt16(_)  => "int",
            BfpType::UInt32(_)  => "int",
            BfpType::UInt64(_)  => "int",
            BfpType::UInt128(_) => "int",

            BfpType::Int8(_)    => "int",
            BfpType::Int16(_)   => "int",
            BfpType::Int32(_)   => "int",
            BfpType::Int64(_)   => "int",
            BfpType::Int128(_)  => "int",

            BfpType::Float32(_) => "float",
            BfpType::Float64(_) => "float",

            BfpType::Bool8(_)   => "bool",
            BfpType::Bool16(_)  => "bool",
            BfpType::Bool32(_)  => "bool",
            BfpType::Bool64(_)  => "bool",
            BfpType::Bool128(_) => "bool",

            BfpType::Struct(_)  => "BaseStruct"
        }.into()
    }

    pub fn to_parseable_from_usize(&self, value: usize) -> Option<ParseableType> {
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
            BfpType::Int128(_)  => { Some(ParseableType::Int128(value as i128)) },
            
            _                   => { None }
        }
    }
    
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

            BfpType::Struct(struct_) => {
                let py_type = struct_.py_type.bind(value.py());
                if !value.is_exact_instance(py_type) {
                    return Err(PyTypeError::new_err(
                        format!(
                            "'{}' object cannot be interpreted as a '{}'",
                            value.get_type().fully_qualified_name()?.to_string(),
                            py_type.fully_qualified_name()?.to_string()
                        )
                    ))
                }
                
                ParseableType::Struct(value.extract::<BaseStruct>()?, struct_.py_type.clone())
            }
        })
    }
}

impl Parseable for BfpType {
    type Type = ParseableType;

    fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> std::io::Result<Self::Type> {
        Ok(match self {
            BfpType::UInt8(val)   => val.from_stream(stream, ver)?.into(),
            BfpType::UInt16(val)  => val.from_stream(stream, ver)?.into(),
            BfpType::UInt32(val)  => val.from_stream(stream, ver)?.into(),
            BfpType::UInt64(val)  => val.from_stream(stream, ver)?.into(),
            BfpType::UInt128(val) => val.from_stream(stream, ver)?.into(),
            
            BfpType::Int8(val)    => val.from_stream(stream, ver)?.into(),
            BfpType::Int16(val)   => val.from_stream(stream, ver)?.into(),
            BfpType::Int32(val)   => val.from_stream(stream, ver)?.into(),
            BfpType::Int64(val)   => val.from_stream(stream, ver)?.into(),
            BfpType::Int128(val)  => val.from_stream(stream, ver)?.into(),
            
            BfpType::Float32(val) => val.from_stream(stream, ver)?.into(),
            BfpType::Float64(val) => val.from_stream(stream, ver)?.into(),
            
            BfpType::Bool8(val)   => val.from_stream(stream, ver)?.into(),
            BfpType::Bool16(val)  => val.from_stream(stream, ver)?.into(),
            BfpType::Bool32(val)  => val.from_stream(stream, ver)?.into(),
            BfpType::Bool64(val)  => val.from_stream(stream, ver)?.into(),
            BfpType::Bool128(val) => val.from_stream(stream, ver)?.into(),

            BfpType::Struct(val)  => ParseableType::Struct(val.from_stream(stream, ver)?, val.py_type.clone()),
        })
    }

    fn to_bytes(&self, _value: &Self::Type) -> Vec<u8> {
        todo!()
    }
}