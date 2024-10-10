use std::sync::Arc;

use pyo3::types::PyType;
use pyo3::{Bound, IntoPy, Py, PyAny, Python};

use crate::impl_from_for_parseable_type;
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_list::BfpList;

#[derive(Debug, Clone)]
pub enum ParseableType {
    None,
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
    
    Array(BfpList),
    
    Bytes(Vec<u8>),
    
    Option(Option<Box<ParseableType>>),
    
    Struct(BaseStruct, Arc<Py<PyType>>),
}

// impl PartialOrd for ParseableType {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         match (self, other) { // todo fix
//             (ParseableType::UInt8(val1), ParseableType::UInt8(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::UInt16(val1), ParseableType::UInt16(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::UInt32(val1), ParseableType::UInt32(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::UInt64(val1), ParseableType::UInt64(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::UInt128(val1), ParseableType::UInt128(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::Int8(val1), ParseableType::Int8(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::Int16(val1), ParseableType::Int16(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::Int32(val1), ParseableType::Int32(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::Int64(val1), ParseableType::Int64(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::Int128(val1), ParseableType::Int128(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::Float32(val1), ParseableType::Float32(val2)) => val1.partial_cmp(val2),
//             (ParseableType::Float64(val1), ParseableType::Float64(val2)) => val1.partial_cmp(val2),
//             (ParseableType::Bool(val1), ParseableType::Bool(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::Str(val1), ParseableType::Str(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::Array(val1), ParseableType::Array(val2)) => None,
//             (ParseableType::Bytes(val1), ParseableType::Bytes(val2)) => Some(val1.cmp(val2)),
//             (ParseableType::Option(val1), ParseableType::Option(val2)) => val1.partial_cmp(val2),
//             (ParseableType::Struct(val1, _type1), ParseableType::Struct(val2, _type2)) => None,
//             _ => None,
//         }
//     }
// }

impl PartialEq for ParseableType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) { // todo fix
            (ParseableType::None,                 ParseableType::None)                 => true,
            (ParseableType::UInt8(val1),          ParseableType::UInt8(val2))          => val1 == val2,
            (ParseableType::UInt16(val1),         ParseableType::UInt16(val2))         => val1 == val2,
            (ParseableType::UInt32(val1),         ParseableType::UInt32(val2))         => val1 == val2,
            (ParseableType::UInt64(val1),         ParseableType::UInt64(val2))         => val1 == val2,
            (ParseableType::UInt128(val1),        ParseableType::UInt128(val2))        => val1 == val2,

            (ParseableType::Int8(val1),           ParseableType::Int8(val2))           => val1 == val2,
            (ParseableType::Int16(val1),          ParseableType::Int16(val2))          => val1 == val2,
            (ParseableType::Int32(val1),          ParseableType::Int32(val2))          => val1 == val2,
            (ParseableType::Int64(val1),          ParseableType::Int64(val2))          => val1 == val2,
            (ParseableType::Int128(val1),         ParseableType::Int128(val2))         => val1 == val2,

            (ParseableType::Float32(val1),        ParseableType::Float32(val2))        => val1 == val2,
            (ParseableType::Float64(val1),        ParseableType::Float64(val2))        => val1 == val2,

            (ParseableType::Bool(val1),           ParseableType::Bool(val2))           => val1 == val2,

            (ParseableType::Str(val1),            ParseableType::Str(val2))            => val1 == val2,

            (ParseableType::Array(val1),          ParseableType::Array(val2))          => val1 == val2,

            (ParseableType::Bytes(val1),          ParseableType::Bytes(val2))          => val1 == val2,

            (ParseableType::Option(val1),         ParseableType::Option(val2))         => val1 == val2,

            (ParseableType::Struct(val1, _type1), ParseableType::Struct(val2, _type2)) => val1 == val2,
            _                                                                          => false
        }
    }
}

impl Eq for ParseableType {}

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

impl_from_for_parseable_type!(BfpList, Array);

impl_from_for_parseable_type!(Vec<u8>, Bytes);

impl_from_for_parseable_type!(Option<Box<ParseableType>>, Option);

impl ParseableType {
    pub fn to_bound(self, py: Python) -> Bound<'_, PyAny> {
        match self {
            ParseableType::None                 => py.None().into_bound(py),
            ParseableType::UInt8(val)           => val.into_py(py).into_bound(py),
            ParseableType::UInt16(val)          => val.into_py(py).into_bound(py),
            ParseableType::UInt32(val)          => val.into_py(py).into_bound(py),
            ParseableType::UInt64(val)          => val.into_py(py).into_bound(py),
            ParseableType::UInt128(val)         => val.into_py(py).into_bound(py),

            ParseableType::Int8(val)            => val.into_py(py).into_bound(py),
            ParseableType::Int16(val)           => val.into_py(py).into_bound(py),
            ParseableType::Int32(val)           => val.into_py(py).into_bound(py),
            ParseableType::Int64(val)           => val.into_py(py).into_bound(py),
            ParseableType::Int128(val)          => val.into_py(py).into_bound(py),
            ParseableType::Float32(val)         => val.into_py(py).into_bound(py),
            ParseableType::Float64(val)         => val.into_py(py).into_bound(py),

            ParseableType::Bool(val)            => val.into_py(py).into_bound(py),

            ParseableType::Str(val)             => val.into_py(py).into_bound(py),

            ParseableType::Array(val)           => val.into_py(py).into_bound(py),

            ParseableType::Bytes(val)           => val.into_py(py).into_bound(py),

            ParseableType::Option(_val)         => todo!(),

            ParseableType::Struct(val, py_type) => BaseStruct::with_cls(val, py_type.bind(py)),
        }
    }
}