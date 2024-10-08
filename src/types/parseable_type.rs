use std::sync::Arc;
use pyo3::{Bound, IntoPy, Py, PyAny, PyResult, Python};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::{PyAnyMethods, PyTypeMethods};
use pyo3::types::PyType;
use crate::impl_from_for_parseable_type;
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_type::BfpType;

#[derive(Debug, Clone)]
pub enum ParseableType {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),

    Float32(f32),
    Float64(f64),

    Bool(bool),
    
    Str(String),
    
    Array(Vec<ParseableType>),
    
    Bytes(Vec<u8>),
    
    Option(Option<Box<ParseableType>>),
    
    Struct(BaseStruct, Arc<Py<PyType>>)
}

impl_from_for_parseable_type!(u8, UInt8);
impl_from_for_parseable_type!(u16, UInt16);
impl_from_for_parseable_type!(u32, UInt32);
impl_from_for_parseable_type!(u64, UInt64);
impl_from_for_parseable_type!(u128, UInt128);

impl_from_for_parseable_type!(i8, Int8);
impl_from_for_parseable_type!(i16, Int16);
impl_from_for_parseable_type!(i32, Int32);
impl_from_for_parseable_type!(i64, Int64);
impl_from_for_parseable_type!(i128, Int128);

impl_from_for_parseable_type!(f32, Float32);
impl_from_for_parseable_type!(f64, Float64);

impl_from_for_parseable_type!(bool, Bool);

impl_from_for_parseable_type!(String, Str);

impl_from_for_parseable_type!(Vec<ParseableType>, Array);

impl_from_for_parseable_type!(Vec<u8>, Bytes);

impl_from_for_parseable_type!(Option<Box<ParseableType>>, Option);

impl ParseableType {
    pub fn to_bound(self, py: Python) -> PyResult<Bound<'_, PyAny>> {
        Ok(match self {
            ParseableType::UInt8(val) => { val.into_py(py).into_bound(py) }
            ParseableType::UInt16(val) => { val.into_py(py).into_bound(py) }
            ParseableType::UInt32(val) => { val.into_py(py).into_bound(py) }
            ParseableType::UInt64(val) => { val.into_py(py).into_bound(py) }
            ParseableType::UInt128(val) => { val.into_py(py).into_bound(py) }
            
            ParseableType::Int8(val) => { val.into_py(py).into_bound(py) }
            ParseableType::Int16(val) => { val.into_py(py).into_bound(py) }
            ParseableType::Int32(val) => { val.into_py(py).into_bound(py) }
            ParseableType::Int64(val) => { val.into_py(py).into_bound(py) }
            ParseableType::Int128(val) => { val.into_py(py).into_bound(py) }
            ParseableType::Float32(val) => { val.into_py(py).into_bound(py) }
            ParseableType::Float64(val) => { val.into_py(py).into_bound(py) }
            
            ParseableType::Bool(val) => { val.into_py(py).into_bound(py) }
            
            ParseableType::Str(val) => { val.into_py(py).into_bound(py) }
            
            ParseableType::Array(val) => { todo!() }
            
            ParseableType::Bytes(val) => { val.into_py(py).into_bound(py) }
            
            ParseableType::Option(val) => { todo!() }
            
            ParseableType::Struct(val, py_type) => { BaseStruct::with_cls(val, py_type.bind(py))? }
        })
    }
    
    pub fn from_bound(value: &Bound<'_, PyAny>, data_type: &BfpType) -> PyResult<Self> {
        Ok(match data_type {
            BfpType::UInt8(_) => { value.extract::<u8>()?.into() }
            BfpType::UInt16(_) => { value.extract::<u16>()?.into() }
            BfpType::UInt32(_) => { value.extract::<u32>()?.into() }
            BfpType::UInt64(_) => { value.extract::<u64>()?.into() }
            BfpType::UInt128(_) => { value.extract::<u128>()?.into() }
            
            BfpType::Int8(_) => { value.extract::<i8>()?.into() }
            BfpType::Int16(_) => { value.extract::<i16>()?.into() }
            BfpType::Int32(_) => { value.extract::<i32>()?.into() }
            BfpType::Int64(_) => { value.extract::<i64>()?.into() }
            BfpType::Int128(_) => { value.extract::<i128>()?.into() }
            
            BfpType::Float32(_) => { value.extract::<f32>()?.into() }
            BfpType::Float64(_) => { value.extract::<f64>()?.into() }
            
            BfpType::Bool8(_) => { value.extract::<bool>()?.into() }
            BfpType::Bool16(_) => { value.extract::<bool>()?.into() }
            BfpType::Bool32(_) => { value.extract::<bool>()?.into() }
            BfpType::Bool64(_) => { value.extract::<bool>()?.into() }
            BfpType::Bool128(_) => { value.extract::<bool>()?.into() }
            
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