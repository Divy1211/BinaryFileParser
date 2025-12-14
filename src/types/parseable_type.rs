use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use pyo3::{Bound, IntoPyObjectExt, PyAny, PyResult, Python};
use pyo3::types::PyBytes;
use serde::{Serialize, Serializer};

use crate::{impl_from_for_parseable_type, impl_try_into_for_parseable_type};
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_list::BfpList;
use crate::types::bfp_type::BfpType;
use crate::types::diff::diff::{Diff, Diffable, IDiff};
use crate::types::diff::merge::{Conflict, Mergeable};
use crate::types::r#struct::Struct;
use crate::types::serial::struct_serializer::StructSerializer;
use crate::types::diff::struct_diffable::StructDiffable;
use crate::types::diff::struct_mergeable::StructMergeable;

// todo: change to structural enum
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
    
    Struct { val: BaseStruct, struct_: Struct },
}

impl ParseableType {
    pub fn is_ls_of(&self, bfp_type: &BfpType) -> bool {
        match self {
            ParseableType::Array(val) => {
                let inner = val.inner();
                inner.data_type == *bfp_type
            },
            _ => false,
        }
    }
    
    /// converts ParseableTypes back to python values
    pub fn to_bound(self, py: Python) -> PyResult<Bound<'_, PyAny>> {
        match self {
            ParseableType::None                         => Ok(py.None().into_bound(py)),
            ParseableType::UInt8(val)                   => Ok(val.into_bound_py_any(py)?),
            ParseableType::UInt16(val)                  => Ok(val.into_bound_py_any(py)?),
            ParseableType::UInt32(val)                  => Ok(val.into_bound_py_any(py)?),
            ParseableType::UInt64(val)                  => Ok(val.into_bound_py_any(py)?),
            ParseableType::UInt128(val)                 => Ok(val.into_bound_py_any(py)?),

            ParseableType::Int8(val)                    => Ok(val.into_bound_py_any(py)?),
            ParseableType::Int16(val)                   => Ok(val.into_bound_py_any(py)?),
            ParseableType::Int32(val)                   => Ok(val.into_bound_py_any(py)?),
            ParseableType::Int64(val)                   => Ok(val.into_bound_py_any(py)?),
            ParseableType::Int128(val)                  => Ok(val.into_bound_py_any(py)?),

            ParseableType::Float32(val)                 => Ok(val.into_bound_py_any(py)?),
            ParseableType::Float64(val)                 => Ok(val.into_bound_py_any(py)?),

            ParseableType::Bool(val)                    => Ok(val.into_bound_py_any(py)?),

            ParseableType::Str(val)                     => Ok(val.into_bound_py_any(py)?),

            ParseableType::Array(val)                   => Ok(val.into_bound_py_any(py)?),

            ParseableType::Bytes(val)                   => Ok(PyBytes::new(py, &val).into_any()),

            ParseableType::Option(val)                  => { 
                match val {
                    None      => Ok(py.None().into_bound(py)),
                    Some(val) => val.to_bound(py),
                }
            },

            ParseableType::Struct { val, struct_ }      => {
                let inner = val.inner();
                match inner.obj.get() {
                    None => {
                        let obj = BaseStruct::with_cls(val.clone(), struct_.py_type(py))?;
                        inner.obj.set(obj.clone().unbind()).expect("infallible");
                        Ok(obj)
                    }
                    Some(obj) => {
                        Ok(obj.bind(py).clone())
                    }
                }
            },
        }
    }
    
    pub fn try_to_int(&self) -> Option<i128> {
        match self {
            ParseableType::UInt8(val)   => Some(*val as i128),
            ParseableType::UInt16(val)  => Some(*val as i128),
            ParseableType::UInt32(val)  => Some(*val as i128),
            ParseableType::UInt64(val)  => Some(*val as i128),
            ParseableType::UInt128(val) => Some(*val as i128),

            ParseableType::Int8(val)    => Some(*val as i128),
            ParseableType::Int16(val)   => Some(*val as i128),
            ParseableType::Int32(val)   => Some(*val as i128),
            ParseableType::Int64(val)   => Some(*val as i128),
            ParseableType::Int128(val)  => Some(*val),
            
            _ => None,
        }
    }
    
    pub fn try_to_float(&self) -> Option<f64> {
        match self {
            ParseableType::Float32(val) => Some(*val as f64),
            ParseableType::Float64(val) => Some(*val),

            _ => None,
        }
    }
    
    pub fn try_len(&self) -> Option<usize> {
        match self {
            ParseableType::Array(ls) => Some(ls.len()),
            _ => None,
        }
    }
}

impl PartialOrd for ParseableType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.try_to_int(), other.try_to_int()) {
            (Some(num1), Some(num2))          => return num1.partial_cmp(&num2),
            (Some(_), None) | (None, Some(_)) => return None,
            _ => {}
        };

        match (self.try_to_float(), other.try_to_float()) {
            (Some(num1), Some(num2))          => return num1.partial_cmp(&num2),
            (Some(_), None) | (None, Some(_)) => return None,
            _ => {}
        };

        match (self, other) { // todo fix
            (ParseableType::None,          ParseableType::None)          => Some(Ordering::Equal),
            (ParseableType::Bool(val1),    ParseableType::Bool(val2))    => val1.partial_cmp(val2),
            (ParseableType::Str(val1),     ParseableType::Str(val2))     => val1.partial_cmp(val2),
            (ParseableType::Array(val1),   ParseableType::Array(val2))   => val1.partial_cmp(val2),
            (ParseableType::Bytes(val1),   ParseableType::Bytes(val2))   => val1.partial_cmp(val2),
            (ParseableType::Option(val1),  ParseableType::Option(val2))  => val1.partial_cmp(val2),
            (ParseableType::Struct { .. }, ParseableType::Struct { .. }) => None,
            _                                                            => None
        }
    }
}

impl PartialEq for ParseableType {
    fn eq(&self, other: &Self) -> bool {
        match (self.try_to_int(), other.try_to_int()) {
            (Some(num1), Some(num2))          => return num1 == num2,
            (Some(_), None) | (None, Some(_)) => return false,
            _ => {}
        };

        match (self.try_to_float(), other.try_to_float()) {
            (Some(num1), Some(num2))          => return num1 == num2,
            (Some(_), None) | (None, Some(_)) => return false,
            _ => {}
        };
        
        match (self, other) { // todo fix
            (ParseableType::None,                     ParseableType::None)                     => true,
            (ParseableType::Bool(val1),               ParseableType::Bool(val2))               => val1 == val2,
            (ParseableType::Str(val1),                ParseableType::Str(val2))                => val1 == val2,
            (ParseableType::Array(val1),              ParseableType::Array(val2))              => val1 == val2,
            (ParseableType::Bytes(val1),              ParseableType::Bytes(val2))              => val1 == val2,
            (ParseableType::Option(val1),             ParseableType::Option(val2))             => val1 == val2,
            (ParseableType::Struct { val: val1, .. }, ParseableType::Struct { val: val2, .. }) => val1 == val2,
            _                                                                                  => false
        }
    }
}

impl Eq for ParseableType {}

impl_try_into_for_parseable_type!(isize);
impl_try_into_for_parseable_type!(usize);

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

impl Serialize for ParseableType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            ParseableType::None => serializer.serialize_none(),

            ParseableType::UInt8(v) => serializer.serialize_u8(*v),
            ParseableType::UInt16(v) => serializer.serialize_u16(*v),
            ParseableType::UInt32(v) => serializer.serialize_u32(*v),
            ParseableType::UInt64(v) => serializer.serialize_u64(*v),
            ParseableType::UInt128(v) => serializer.serialize_u128(*v),

            ParseableType::Int8(v) => serializer.serialize_i8(*v),
            ParseableType::Int16(v) => serializer.serialize_i16(*v),
            ParseableType::Int32(v) => serializer.serialize_i32(*v),
            ParseableType::Int64(v) => serializer.serialize_i64(*v),
            ParseableType::Int128(v) => serializer.serialize_i128(*v),

            ParseableType::Float32(v) => serializer.serialize_f32(*v),
            ParseableType::Float64(v) => serializer.serialize_f64(*v),

            ParseableType::Bool(v) => serializer.serialize_bool(*v),

            ParseableType::Str(s) => serializer.serialize_str(s),

            ParseableType::Array(arr) => arr.serialize(serializer),

            ParseableType::Bytes(bytes) => serializer.serialize_bytes(bytes),

            ParseableType::Option(opt) => opt.serialize(serializer),

            ParseableType::Struct { val, struct_, .. } => {
                StructSerializer(struct_, val).serialize(serializer)
            }
        }
    }
}

impl Hash for ParseableType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);

        match self {
            ParseableType::None => {}
            ParseableType::UInt8(v) => v.hash(state),
            ParseableType::UInt16(v) => v.hash(state),
            ParseableType::UInt32(v) => v.hash(state),
            ParseableType::UInt64(v) => v.hash(state),
            ParseableType::UInt128(v) => v.hash(state),

            ParseableType::Int8(v) => v.hash(state),
            ParseableType::Int16(v) => v.hash(state),
            ParseableType::Int32(v) => v.hash(state),
            ParseableType::Int64(v) => v.hash(state),
            ParseableType::Int128(v) => v.hash(state),

            ParseableType::Float32(v) => {
                v.to_bits().hash(state);
            }
            ParseableType::Float64(v) => {
                v.to_bits().hash(state);
            }

            ParseableType::Bool(v) => v.hash(state),
            ParseableType::Str(s) => s.hash(state),
            ParseableType::Array(arr) => arr.hash(state),
            ParseableType::Bytes(bytes) => bytes.hash(state),
            ParseableType::Option(opt) => opt.hash(state),
            ParseableType::Struct { val, .. } => {
                val.hash(state);
            }
        }
    }
}


impl Diffable<ParseableType> for ParseableType {
    fn diff(&self, other: &ParseableType) -> Diff<ParseableType> {
        match (self, other) {
            (ParseableType::None, ParseableType::None) => Diff::None,
            (ParseableType::None, _)                   => Diff::Inserted(other.clone()),
            (_, ParseableType::None)                   => Diff::Deleted(other.clone()),
            
            (ParseableType::UInt8(v1),     ParseableType::UInt8(v2))   => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },
            (ParseableType::UInt16(v1),    ParseableType::UInt16(v2))  => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },
            (ParseableType::UInt32(v1),    ParseableType::UInt32(v2))  => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },
            (ParseableType::UInt64(v1),    ParseableType::UInt64(v2))  => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },
            (ParseableType::UInt128(v1),   ParseableType::UInt128(v2)) => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },

            (ParseableType::Int8(v1),      ParseableType::Int8(v2))    => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },
            (ParseableType::Int16(v1),     ParseableType::Int16(v2))   => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },
            (ParseableType::Int32(v1),     ParseableType::Int32(v2))   => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },
            (ParseableType::Int64(v1),     ParseableType::Int64(v2))   => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },
            (ParseableType::Int128(v1),    ParseableType::Int128(v2))  => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },

            (ParseableType::Float32(v1),   ParseableType::Float32(v2)) => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },
            (ParseableType::Float64(v1),   ParseableType::Float64(v2)) => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },

            (ParseableType::Bool(v1),      ParseableType::Bool(v2))    => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },

            (ParseableType::Str(v1),       ParseableType::Str(v2))     => if v1 == v2 { Diff::None } else { Diff::Changed(other.clone()) },

            (ParseableType::Array(arr1),   ParseableType::Array(arr2)) => {
                arr1.diff(arr2)
            },

            // this is a bytestring, no recursive diff needed
            (ParseableType::Bytes(bytes1), ParseableType::Bytes(bytes2)) => if bytes1 == bytes2 { Diff::None } else { Diff::Changed(other.clone()) },

            (ParseableType::Option(opt1), ParseableType::Option(opt2)) => {
                match (opt1, opt2) {
                    (None, None) => Diff::None,
                    (Some(v1), Some(v2)) if v1 == v2 => Diff::None,
                    (Some(v1), Some(v2)) => v1.diff(v2),
                    _ => Diff::Changed(other.clone()),
                }
            },

            (ParseableType::Struct { val: val1, struct_: struct1, .. }, ParseableType::Struct { val: val2, struct_: struct2, .. }) => {
                StructDiffable(struct1, val1).diff(&StructDiffable(struct2, val2))
            }
            _ => unreachable!("BFP Internal Error: unhandled types or diffing two different types"),
        }
    }
}

impl Mergeable<ParseableType> for ParseableType {
    fn patch(&mut self, change: IDiff<ParseableType>, off: isize) -> isize {
        match self {
            ParseableType::Array(ls) => {
                ls.patch(change, off)
            }
            ParseableType::Option(val) => {
                match val.as_mut() {
                    None => {
                        *val = change.1.value().map(Box::new);
                        off
                    }
                    Some(val) => {
                        val.patch(change, off)
                    }
                }
            }
            ParseableType::Struct { val, struct_ } => {
                StructMergeable(struct_, val).patch(change, isize::MIN);
                off
            }
            _ => { unreachable!("BFP Internal Error: patch called on trivial ParseableType") }
        }
    }

    fn merge(&mut self, _slf: &Self, _other: &Self) -> Vec<Conflict<ParseableType>> {
        unreachable!("BFP Internal Error: merge called on ParseableType")
    }
}

impl ParseableType {
    pub fn merge_rec(
        &mut self,
        changes1: Vec<IDiff<ParseableType>>,
        changes2: Vec<IDiff<ParseableType>>,
        conflicts: &mut Vec<Conflict<ParseableType>>,
    ) {
        match self {
            ParseableType::Array(ls) => {
                ls.merge_rec(changes1, changes2, conflicts);
            }
            ParseableType::Option(val) => {
                if let Some(val) = val.as_mut() {
                    val.merge_rec(changes1, changes2, conflicts)
                }
            }
            ParseableType::Struct { val, struct_ } => {
                StructMergeable(struct_, val).merge_rec(changes1, changes2, conflicts);
            }
            _ => { unreachable!("BFP Internal Error: merge_rec called on trivial ParseableType") }
        }
    }
}