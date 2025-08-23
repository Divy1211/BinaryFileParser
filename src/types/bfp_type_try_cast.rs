use crate::types::bfp_type::BfpType;
use crate::types::parseable_type::ParseableType;

impl BfpType {
    pub fn try_cast(&self, value: ParseableType) -> Option<ParseableType> {
        match (self, &value) {
            (BfpType::UInt8(_type), ParseableType::UInt8(val))     => Some(ParseableType::UInt8(*val as u8)),
            (BfpType::UInt8(_type), ParseableType::UInt16(val))    => Some(ParseableType::UInt8(*val as u8)),
            (BfpType::UInt8(_type), ParseableType::UInt32(val))    => Some(ParseableType::UInt8(*val as u8)),
            (BfpType::UInt8(_type), ParseableType::UInt64(val))    => Some(ParseableType::UInt8(*val as u8)),
            (BfpType::UInt8(_type), ParseableType::UInt128(val))   => Some(ParseableType::UInt8(*val as u8)),

            (BfpType::UInt8(_type), ParseableType::Int8(val))      => Some(ParseableType::UInt8(*val as u8)),
            (BfpType::UInt8(_type), ParseableType::Int16(val))     => Some(ParseableType::UInt8(*val as u8)),
            (BfpType::UInt8(_type), ParseableType::Int32(val))     => Some(ParseableType::UInt8(*val as u8)),
            (BfpType::UInt8(_type), ParseableType::Int64(val))     => Some(ParseableType::UInt8(*val as u8)),
            (BfpType::UInt8(_type), ParseableType::Int128(val))    => Some(ParseableType::UInt8(*val as u8)),

            (BfpType::UInt8(_type), ParseableType::Float32(val))   => Some(ParseableType::UInt8(*val as u8)),
            (BfpType::UInt8(_type), ParseableType::Float64(val))   => Some(ParseableType::UInt8(*val as u8)),

            (BfpType::UInt8(_type), ParseableType::Bool(val))      => Some(ParseableType::UInt8(*val as u8)),

            (BfpType::UInt16(_type), ParseableType::UInt8(val))    => Some(ParseableType::UInt16(*val as u16)),
            (BfpType::UInt16(_type), ParseableType::UInt16(val))   => Some(ParseableType::UInt16(*val as u16)),
            (BfpType::UInt16(_type), ParseableType::UInt32(val))   => Some(ParseableType::UInt16(*val as u16)),
            (BfpType::UInt16(_type), ParseableType::UInt64(val))   => Some(ParseableType::UInt16(*val as u16)),
            (BfpType::UInt16(_type), ParseableType::UInt128(val))  => Some(ParseableType::UInt16(*val as u16)),

            (BfpType::UInt16(_type), ParseableType::Int8(val))     => Some(ParseableType::UInt16(*val as u16)),
            (BfpType::UInt16(_type), ParseableType::Int16(val))    => Some(ParseableType::UInt16(*val as u16)),
            (BfpType::UInt16(_type), ParseableType::Int32(val))    => Some(ParseableType::UInt16(*val as u16)),
            (BfpType::UInt16(_type), ParseableType::Int64(val))    => Some(ParseableType::UInt16(*val as u16)),
            (BfpType::UInt16(_type), ParseableType::Int128(val))   => Some(ParseableType::UInt16(*val as u16)),

            (BfpType::UInt16(_type), ParseableType::Float32(val))  => Some(ParseableType::UInt16(*val as u16)),
            (BfpType::UInt16(_type), ParseableType::Float64(val))  => Some(ParseableType::UInt16(*val as u16)),

            (BfpType::UInt16(_type), ParseableType::Bool(val))     => Some(ParseableType::UInt16(*val as u16)),

            (BfpType::UInt32(_type), ParseableType::UInt8(val))    => Some(ParseableType::UInt32(*val as u32)),
            (BfpType::UInt32(_type), ParseableType::UInt16(val))   => Some(ParseableType::UInt32(*val as u32)),
            (BfpType::UInt32(_type), ParseableType::UInt32(val))   => Some(ParseableType::UInt32(*val as u32)),
            (BfpType::UInt32(_type), ParseableType::UInt64(val))   => Some(ParseableType::UInt32(*val as u32)),
            (BfpType::UInt32(_type), ParseableType::UInt128(val))  => Some(ParseableType::UInt32(*val as u32)),

            (BfpType::UInt32(_type), ParseableType::Int8(val))     => Some(ParseableType::UInt32(*val as u32)),
            (BfpType::UInt32(_type), ParseableType::Int16(val))    => Some(ParseableType::UInt32(*val as u32)),
            (BfpType::UInt32(_type), ParseableType::Int32(val))    => Some(ParseableType::UInt32(*val as u32)),
            (BfpType::UInt32(_type), ParseableType::Int64(val))    => Some(ParseableType::UInt32(*val as u32)),
            (BfpType::UInt32(_type), ParseableType::Int128(val))   => Some(ParseableType::UInt32(*val as u32)),

            (BfpType::UInt32(_type), ParseableType::Float32(val))  => Some(ParseableType::UInt32(*val as u32)),
            (BfpType::UInt32(_type), ParseableType::Float64(val))  => Some(ParseableType::UInt32(*val as u32)),

            (BfpType::UInt32(_type), ParseableType::Bool(val))     => Some(ParseableType::UInt32(*val as u32)),

            (BfpType::UInt64(_type), ParseableType::UInt8(val))    => Some(ParseableType::UInt64(*val as u64)),
            (BfpType::UInt64(_type), ParseableType::UInt16(val))   => Some(ParseableType::UInt64(*val as u64)),
            (BfpType::UInt64(_type), ParseableType::UInt32(val))   => Some(ParseableType::UInt64(*val as u64)),
            (BfpType::UInt64(_type), ParseableType::UInt64(val))   => Some(ParseableType::UInt64(*val as u64)),
            (BfpType::UInt64(_type), ParseableType::UInt128(val))  => Some(ParseableType::UInt64(*val as u64)),

            (BfpType::UInt64(_type), ParseableType::Int8(val))     => Some(ParseableType::UInt64(*val as u64)),
            (BfpType::UInt64(_type), ParseableType::Int16(val))    => Some(ParseableType::UInt64(*val as u64)),
            (BfpType::UInt64(_type), ParseableType::Int32(val))    => Some(ParseableType::UInt64(*val as u64)),
            (BfpType::UInt64(_type), ParseableType::Int64(val))    => Some(ParseableType::UInt64(*val as u64)),
            (BfpType::UInt64(_type), ParseableType::Int128(val))   => Some(ParseableType::UInt64(*val as u64)),

            (BfpType::UInt64(_type), ParseableType::Float32(val))  => Some(ParseableType::UInt64(*val as u64)),
            (BfpType::UInt64(_type), ParseableType::Float64(val))  => Some(ParseableType::UInt64(*val as u64)),

            (BfpType::UInt64(_type), ParseableType::Bool(val))     => Some(ParseableType::UInt64(*val as u64)),

            (BfpType::UInt128(_type), ParseableType::UInt8(val))   => Some(ParseableType::UInt128(*val as u128)),
            (BfpType::UInt128(_type), ParseableType::UInt16(val))  => Some(ParseableType::UInt128(*val as u128)),
            (BfpType::UInt128(_type), ParseableType::UInt32(val))  => Some(ParseableType::UInt128(*val as u128)),
            (BfpType::UInt128(_type), ParseableType::UInt64(val))  => Some(ParseableType::UInt128(*val as u128)),
            (BfpType::UInt128(_type), ParseableType::UInt128(val)) => Some(ParseableType::UInt128(*val as u128)),

            (BfpType::UInt128(_type), ParseableType::Int8(val))    => Some(ParseableType::UInt128(*val as u128)),
            (BfpType::UInt128(_type), ParseableType::Int16(val))   => Some(ParseableType::UInt128(*val as u128)),
            (BfpType::UInt128(_type), ParseableType::Int32(val))   => Some(ParseableType::UInt128(*val as u128)),
            (BfpType::UInt128(_type), ParseableType::Int64(val))   => Some(ParseableType::UInt128(*val as u128)),
            (BfpType::UInt128(_type), ParseableType::Int128(val))  => Some(ParseableType::UInt128(*val as u128)),

            (BfpType::UInt128(_type), ParseableType::Float32(val)) => Some(ParseableType::UInt128(*val as u128)),
            (BfpType::UInt128(_type), ParseableType::Float64(val)) => Some(ParseableType::UInt128(*val as u128)),

            (BfpType::UInt128(_type), ParseableType::Bool(val))    => Some(ParseableType::UInt128(*val as u128)),


            (BfpType::Int8(_type), ParseableType::UInt8(val))      => Some(ParseableType::Int8(*val as i8)),
            (BfpType::Int8(_type), ParseableType::UInt16(val))     => Some(ParseableType::Int8(*val as i8)),
            (BfpType::Int8(_type), ParseableType::UInt32(val))     => Some(ParseableType::Int8(*val as i8)),
            (BfpType::Int8(_type), ParseableType::UInt64(val))     => Some(ParseableType::Int8(*val as i8)),
            (BfpType::Int8(_type), ParseableType::UInt128(val))    => Some(ParseableType::Int8(*val as i8)),

            (BfpType::Int8(_type), ParseableType::Int8(val))       => Some(ParseableType::Int8(*val as i8)),
            (BfpType::Int8(_type), ParseableType::Int16(val))      => Some(ParseableType::Int8(*val as i8)),
            (BfpType::Int8(_type), ParseableType::Int32(val))      => Some(ParseableType::Int8(*val as i8)),
            (BfpType::Int8(_type), ParseableType::Int64(val))      => Some(ParseableType::Int8(*val as i8)),
            (BfpType::Int8(_type), ParseableType::Int128(val))     => Some(ParseableType::Int8(*val as i8)),

            (BfpType::Int8(_type), ParseableType::Float32(val))    => Some(ParseableType::Int8(*val as i8)),
            (BfpType::Int8(_type), ParseableType::Float64(val))    => Some(ParseableType::Int8(*val as i8)),

            (BfpType::Int8(_type), ParseableType::Bool(val))       => Some(ParseableType::Int8(*val as i8)),

            (BfpType::Int16(_type), ParseableType::UInt8(val))     => Some(ParseableType::Int16(*val as i16)),
            (BfpType::Int16(_type), ParseableType::UInt16(val))    => Some(ParseableType::Int16(*val as i16)),
            (BfpType::Int16(_type), ParseableType::UInt32(val))    => Some(ParseableType::Int16(*val as i16)),
            (BfpType::Int16(_type), ParseableType::UInt64(val))    => Some(ParseableType::Int16(*val as i16)),
            (BfpType::Int16(_type), ParseableType::UInt128(val))   => Some(ParseableType::Int16(*val as i16)),

            (BfpType::Int16(_type), ParseableType::Int8(val))      => Some(ParseableType::Int16(*val as i16)),
            (BfpType::Int16(_type), ParseableType::Int16(val))     => Some(ParseableType::Int16(*val as i16)),
            (BfpType::Int16(_type), ParseableType::Int32(val))     => Some(ParseableType::Int16(*val as i16)),
            (BfpType::Int16(_type), ParseableType::Int64(val))     => Some(ParseableType::Int16(*val as i16)),
            (BfpType::Int16(_type), ParseableType::Int128(val))    => Some(ParseableType::Int16(*val as i16)),

            (BfpType::Int16(_type), ParseableType::Float32(val))   => Some(ParseableType::Int16(*val as i16)),
            (BfpType::Int16(_type), ParseableType::Float64(val))   => Some(ParseableType::Int16(*val as i16)),

            (BfpType::Int16(_type), ParseableType::Bool(val))      => Some(ParseableType::Int16(*val as i16)),

            (BfpType::Int32(_type), ParseableType::UInt8(val))     => Some(ParseableType::Int32(*val as i32)),
            (BfpType::Int32(_type), ParseableType::UInt16(val))    => Some(ParseableType::Int32(*val as i32)),
            (BfpType::Int32(_type), ParseableType::UInt32(val))    => Some(ParseableType::Int32(*val as i32)),
            (BfpType::Int32(_type), ParseableType::UInt64(val))    => Some(ParseableType::Int32(*val as i32)),
            (BfpType::Int32(_type), ParseableType::UInt128(val))   => Some(ParseableType::Int32(*val as i32)),

            (BfpType::Int32(_type), ParseableType::Int8(val))      => Some(ParseableType::Int32(*val as i32)),
            (BfpType::Int32(_type), ParseableType::Int16(val))     => Some(ParseableType::Int32(*val as i32)),
            (BfpType::Int32(_type), ParseableType::Int32(val))     => Some(ParseableType::Int32(*val as i32)),
            (BfpType::Int32(_type), ParseableType::Int64(val))     => Some(ParseableType::Int32(*val as i32)),
            (BfpType::Int32(_type), ParseableType::Int128(val))    => Some(ParseableType::Int32(*val as i32)),

            (BfpType::Int32(_type), ParseableType::Float32(val))   => Some(ParseableType::Int32(*val as i32)),
            (BfpType::Int32(_type), ParseableType::Float64(val))   => Some(ParseableType::Int32(*val as i32)),

            (BfpType::Int32(_type), ParseableType::Bool(val))      => Some(ParseableType::Int32(*val as i32)),

            (BfpType::Int64(_type), ParseableType::UInt8(val))     => Some(ParseableType::Int64(*val as i64)),
            (BfpType::Int64(_type), ParseableType::UInt16(val))    => Some(ParseableType::Int64(*val as i64)),
            (BfpType::Int64(_type), ParseableType::UInt32(val))    => Some(ParseableType::Int64(*val as i64)),
            (BfpType::Int64(_type), ParseableType::UInt64(val))    => Some(ParseableType::Int64(*val as i64)),
            (BfpType::Int64(_type), ParseableType::UInt128(val))   => Some(ParseableType::Int64(*val as i64)),

            (BfpType::Int64(_type), ParseableType::Int8(val))      => Some(ParseableType::Int64(*val as i64)),
            (BfpType::Int64(_type), ParseableType::Int16(val))     => Some(ParseableType::Int64(*val as i64)),
            (BfpType::Int64(_type), ParseableType::Int32(val))     => Some(ParseableType::Int64(*val as i64)),
            (BfpType::Int64(_type), ParseableType::Int64(val))     => Some(ParseableType::Int64(*val as i64)),
            (BfpType::Int64(_type), ParseableType::Int128(val))    => Some(ParseableType::Int64(*val as i64)),

            (BfpType::Int64(_type), ParseableType::Float32(val))   => Some(ParseableType::Int64(*val as i64)),
            (BfpType::Int64(_type), ParseableType::Float64(val))   => Some(ParseableType::Int64(*val as i64)),

            (BfpType::Int64(_type), ParseableType::Bool(val))      => Some(ParseableType::Int64(*val as i64)),

            (BfpType::Int128(_type), ParseableType::UInt8(val))    => Some(ParseableType::Int128(*val as i128)),
            (BfpType::Int128(_type), ParseableType::UInt16(val))   => Some(ParseableType::Int128(*val as i128)),
            (BfpType::Int128(_type), ParseableType::UInt32(val))   => Some(ParseableType::Int128(*val as i128)),
            (BfpType::Int128(_type), ParseableType::UInt64(val))   => Some(ParseableType::Int128(*val as i128)),
            (BfpType::Int128(_type), ParseableType::UInt128(val))  => Some(ParseableType::Int128(*val as i128)),

            (BfpType::Int128(_type), ParseableType::Int8(val))     => Some(ParseableType::Int128(*val as i128)),
            (BfpType::Int128(_type), ParseableType::Int16(val))    => Some(ParseableType::Int128(*val as i128)),
            (BfpType::Int128(_type), ParseableType::Int32(val))    => Some(ParseableType::Int128(*val as i128)),
            (BfpType::Int128(_type), ParseableType::Int64(val))    => Some(ParseableType::Int128(*val as i128)),
            (BfpType::Int128(_type), ParseableType::Int128(val))   => Some(ParseableType::Int128(*val as i128)),

            (BfpType::Int128(_type), ParseableType::Float32(val))  => Some(ParseableType::Int128(*val as i128)),
            (BfpType::Int128(_type), ParseableType::Float64(val))  => Some(ParseableType::Int128(*val as i128)),

            (BfpType::Int128(_type), ParseableType::Bool(val))     => Some(ParseableType::Int128(*val as i128)),

            (BfpType::Float32(_type), ParseableType::UInt8(val))   => Some(ParseableType::Float32(*val as f32)),
            (BfpType::Float32(_type), ParseableType::UInt16(val))  => Some(ParseableType::Float32(*val as f32)),
            (BfpType::Float32(_type), ParseableType::UInt32(val))  => Some(ParseableType::Float32(*val as f32)),
            (BfpType::Float32(_type), ParseableType::UInt64(val))  => Some(ParseableType::Float32(*val as f32)),
            (BfpType::Float32(_type), ParseableType::UInt128(val)) => Some(ParseableType::Float32(*val as f32)),

            (BfpType::Float32(_type), ParseableType::Int8(val))    => Some(ParseableType::Float32(*val as f32)),
            (BfpType::Float32(_type), ParseableType::Int16(val))   => Some(ParseableType::Float32(*val as f32)),
            (BfpType::Float32(_type), ParseableType::Int32(val))   => Some(ParseableType::Float32(*val as f32)),
            (BfpType::Float32(_type), ParseableType::Int64(val))   => Some(ParseableType::Float32(*val as f32)),
            (BfpType::Float32(_type), ParseableType::Int128(val))  => Some(ParseableType::Float32(*val as f32)),

            (BfpType::Float32(_type), ParseableType::Float32(val)) => Some(ParseableType::Float32(*val as f32)),
            (BfpType::Float32(_type), ParseableType::Float64(val)) => Some(ParseableType::Float32(*val as f32)),

            (BfpType::Float32(_type), ParseableType::Bool(val))    => Some(ParseableType::Float32(if *val { 1.0 } else { 0.0 })),

            (BfpType::Float64(_type), ParseableType::UInt8(val))   => Some(ParseableType::Float64(*val as f64)),
            (BfpType::Float64(_type), ParseableType::UInt16(val))  => Some(ParseableType::Float64(*val as f64)),
            (BfpType::Float64(_type), ParseableType::UInt32(val))  => Some(ParseableType::Float64(*val as f64)),
            (BfpType::Float64(_type), ParseableType::UInt64(val))  => Some(ParseableType::Float64(*val as f64)),
            (BfpType::Float64(_type), ParseableType::UInt128(val)) => Some(ParseableType::Float64(*val as f64)),

            (BfpType::Float64(_type), ParseableType::Int8(val))    => Some(ParseableType::Float64(*val as f64)),
            (BfpType::Float64(_type), ParseableType::Int16(val))   => Some(ParseableType::Float64(*val as f64)),
            (BfpType::Float64(_type), ParseableType::Int32(val))   => Some(ParseableType::Float64(*val as f64)),
            (BfpType::Float64(_type), ParseableType::Int64(val))   => Some(ParseableType::Float64(*val as f64)),
            (BfpType::Float64(_type), ParseableType::Int128(val))  => Some(ParseableType::Float64(*val as f64)),

            (BfpType::Float64(_type), ParseableType::Float32(val)) => Some(ParseableType::Float64(*val as f64)),
            (BfpType::Float64(_type), ParseableType::Float64(val)) => Some(ParseableType::Float64(*val as f64)),

            (BfpType::Float64(_type), ParseableType::Bool(val))    => Some(ParseableType::Float64(if *val { 1.0 } else { 0.0 })),

            (BfpType::Bool8(_type), ParseableType::UInt8(val))     => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool8(_type), ParseableType::UInt16(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool8(_type), ParseableType::UInt32(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool8(_type), ParseableType::UInt64(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool8(_type), ParseableType::UInt128(val))   => Some(ParseableType::Bool(*val != 0)),

            (BfpType::Bool8(_type), ParseableType::Int8(val))      => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool8(_type), ParseableType::Int16(val))     => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool8(_type), ParseableType::Int32(val))     => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool8(_type), ParseableType::Int64(val))     => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool8(_type), ParseableType::Int128(val))    => Some(ParseableType::Bool(*val != 0)),

            (BfpType::Bool8(_type), ParseableType::Float32(val))   => Some(ParseableType::Bool(*val != 0.0)),
            (BfpType::Bool8(_type), ParseableType::Float64(val))   => Some(ParseableType::Bool(*val != 0.0)),

            (BfpType::Bool8(_type), ParseableType::Bool(val))      => Some(ParseableType::Bool(*val)),

            (BfpType::Bool16(_type), ParseableType::UInt8(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool16(_type), ParseableType::UInt16(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool16(_type), ParseableType::UInt32(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool16(_type), ParseableType::UInt64(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool16(_type), ParseableType::UInt128(val))  => Some(ParseableType::Bool(*val != 0)),

            (BfpType::Bool16(_type), ParseableType::Int8(val))     => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool16(_type), ParseableType::Int16(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool16(_type), ParseableType::Int32(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool16(_type), ParseableType::Int64(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool16(_type), ParseableType::Int128(val))   => Some(ParseableType::Bool(*val != 0)),

            (BfpType::Bool16(_type), ParseableType::Float32(val))  => Some(ParseableType::Bool(*val != 0.0)),
            (BfpType::Bool16(_type), ParseableType::Float64(val))  => Some(ParseableType::Bool(*val != 0.0)),

            (BfpType::Bool16(_type), ParseableType::Bool(val))     => Some(ParseableType::Bool(*val)),

            (BfpType::Bool32(_type), ParseableType::UInt8(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool32(_type), ParseableType::UInt16(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool32(_type), ParseableType::UInt32(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool32(_type), ParseableType::UInt64(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool32(_type), ParseableType::UInt128(val))  => Some(ParseableType::Bool(*val != 0)),

            (BfpType::Bool32(_type), ParseableType::Int8(val))     => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool32(_type), ParseableType::Int16(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool32(_type), ParseableType::Int32(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool32(_type), ParseableType::Int64(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool32(_type), ParseableType::Int128(val))   => Some(ParseableType::Bool(*val != 0)),

            (BfpType::Bool32(_type), ParseableType::Float32(val))  => Some(ParseableType::Bool(*val != 0.0)),
            (BfpType::Bool32(_type), ParseableType::Float64(val))  => Some(ParseableType::Bool(*val != 0.0)),

            (BfpType::Bool32(_type), ParseableType::Bool(val))     => Some(ParseableType::Bool(*val)),

            (BfpType::Bool64(_type), ParseableType::UInt8(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool64(_type), ParseableType::UInt16(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool64(_type), ParseableType::UInt32(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool64(_type), ParseableType::UInt64(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool64(_type), ParseableType::UInt128(val))  => Some(ParseableType::Bool(*val != 0)),

            (BfpType::Bool64(_type), ParseableType::Int8(val))     => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool64(_type), ParseableType::Int16(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool64(_type), ParseableType::Int32(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool64(_type), ParseableType::Int64(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool64(_type), ParseableType::Int128(val))   => Some(ParseableType::Bool(*val != 0)),

            (BfpType::Bool64(_type), ParseableType::Float32(val))  => Some(ParseableType::Bool(*val != 0.0)),
            (BfpType::Bool64(_type), ParseableType::Float64(val))  => Some(ParseableType::Bool(*val != 0.0)),

            (BfpType::Bool64(_type), ParseableType::Bool(val))     => Some(ParseableType::Bool(*val)),

            (BfpType::Bool128(_type), ParseableType::UInt8(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool128(_type), ParseableType::UInt16(val))  => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool128(_type), ParseableType::UInt32(val))  => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool128(_type), ParseableType::UInt64(val))  => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool128(_type), ParseableType::UInt128(val)) => Some(ParseableType::Bool(*val != 0)),

            (BfpType::Bool128(_type), ParseableType::Int8(val))    => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool128(_type), ParseableType::Int16(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool128(_type), ParseableType::Int32(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool128(_type), ParseableType::Int64(val))   => Some(ParseableType::Bool(*val != 0)),
            (BfpType::Bool128(_type), ParseableType::Int128(val))  => Some(ParseableType::Bool(*val != 0)),

            (BfpType::Bool128(_type), ParseableType::Float32(val)) => Some(ParseableType::Bool(*val != 0.0)),
            (BfpType::Bool128(_type), ParseableType::Float64(val)) => Some(ParseableType::Bool(*val != 0.0)),

            (BfpType::Bool128(_type), ParseableType::Bool(val))    => Some(ParseableType::Bool(*val)),

            (BfpType::Bytes(_type),            ParseableType::Bytes(_val))         => Some(value),

            (BfpType::Str(_type),              ParseableType::Str(_val))           => Some(value),
            (BfpType::NTStr(_type),            ParseableType::Str(_val))           => Some(value),

            (BfpType::StrArray(_type),         ParseableType::Array(_val))         => Some(value),

            (BfpType::Option(_type),           ParseableType::Option(_val))        => Some(value),

            (BfpType::Array(_type),            ParseableType::Array(_val))         => Some(value),
            (BfpType::StackedArray(_type),     ParseableType::Array(_val))         => Some(value),
            (BfpType::StackedAttrArray(_type), ParseableType::Array(_val))         => Some(value),

            (BfpType::Struct(type_),   ParseableType::Struct { struct_, .. }) => {
                if type_ == struct_ {
                    Some(value)
                } else {
                    None
                }
            },

            _ => None,
        }
    }
}