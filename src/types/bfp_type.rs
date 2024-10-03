use crate::impl_from_for_bfp_type;
use crate::types::le::bool::{Bool128, Bool16, Bool32, Bool64, Bool8};
use crate::types::le::float::{Float32, Float64};
use crate::types::le::int::{Int128, Int16, Int32, Int64, Int8, UInt128, UInt16, UInt32, UInt64, UInt8};

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
}

impl_from_for_bfp_type!(UInt8, UInt8);
impl_from_for_bfp_type!(UInt16, UInt16);
impl_from_for_bfp_type!(UInt32, UInt32);
impl_from_for_bfp_type!(UInt64, UInt64);
impl_from_for_bfp_type!(UInt128, UInt128);

impl_from_for_bfp_type!(Int8, Int8);
impl_from_for_bfp_type!(Int16, Int16);
impl_from_for_bfp_type!(Int32, Int32);
impl_from_for_bfp_type!(Int64, Int64);
impl_from_for_bfp_type!(Int128, Int128);

impl_from_for_bfp_type!(Float32, Float32);
impl_from_for_bfp_type!(Float64, Float64);

impl_from_for_bfp_type!(Bool8, Bool8);
impl_from_for_bfp_type!(Bool16, Bool16);
impl_from_for_bfp_type!(Bool32, Bool32);
impl_from_for_bfp_type!(Bool64, Bool64);
impl_from_for_bfp_type!(Bool128, Bool128);