use crate::impl_from_for_bfp_type;
use crate::types::le::int16::Int16;
use crate::types::le::int8::Int8;

pub enum BfpType {
    Int8(Int8),
    Int16(Int16),
}

impl_from_for_bfp_type!(Int8, Int8);
impl_from_for_bfp_type!(Int16, Int16);
