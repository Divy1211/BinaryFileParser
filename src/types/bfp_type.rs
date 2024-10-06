use pyo3::pyclass;
use crate::types::byte_stream::ByteStream;
use crate::types::le::bool::{Bool128, Bool16, Bool32, Bool64, Bool8};
use crate::types::le::float::{Float32, Float64};
use crate::types::le::int::{Int128, Int16, Int32, Int64, Int8, UInt128, UInt16, UInt32, UInt64, UInt8};
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::r#struct::Struct;
use crate::types::version::Version;

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

impl Parseable for BfpType {
    type Type = ParseableType;

    fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> std::io::Result<Self::Type> {
        Ok(match self {
            BfpType::UInt8(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::UInt16(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::UInt32(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::UInt64(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::UInt128(val) => { val.from_stream(stream, ver)?.into() }
            
            BfpType::Int8(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::Int16(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::Int32(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::Int64(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::Int128(val) => { val.from_stream(stream, ver)?.into() }
            
            BfpType::Float32(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::Float64(val) => { val.from_stream(stream, ver)?.into() }
            
            BfpType::Bool8(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::Bool16(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::Bool32(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::Bool64(val) => { val.from_stream(stream, ver)?.into() }
            BfpType::Bool128(val) => { val.from_stream(stream, ver)?.into() }
            
            BfpType::Struct(val) => { val.from_stream(stream, ver)?.into() }
        })
    }

    fn to_bytes(&self, value: &Self::Type) -> Vec<u8> {
        todo!()
    }
}