use pyo3::pyclass;
use crate::types::le::bool::{Bool128, Bool16, Bool32, Bool64, Bool8};
use crate::types::le::float::{Float32, Float64};
use crate::types::le::int::{Int128, Int16, Int32, Int64, Int8, UInt128, UInt16, UInt32, UInt64, UInt8};
use crate::types::r#struct::Struct;

#[pyclass]
#[derive(Clone)]
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
