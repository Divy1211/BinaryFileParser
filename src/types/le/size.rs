use std::io::{Error, ErrorKind};
use crate::types::byte_stream::ByteStream;
use crate::types::le::int::{UInt128, UInt16, UInt32, UInt64, UInt8};
use crate::types::parseable::Parseable;
use crate::types::version::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Size {
    UInt8(UInt8),
    UInt16(UInt16),
    UInt32(UInt32),
    UInt64(UInt64),
    UInt128(UInt128),
    Fixed(usize),
}

impl Parseable for Size {
    type Type = usize;

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream(&self, stream: &mut ByteStream, _ver: &Version) -> std::io::Result<Self::Type> {
        Ok(match self {
            Size::UInt8(type_)   => type_.from_stream(stream, _ver)? as usize,
            Size::UInt16(type_)  => type_.from_stream(stream, _ver)? as usize,
            Size::UInt32(type_)  => type_.from_stream(stream, _ver)? as usize,
            Size::UInt64(type_)  => type_.from_stream(stream, _ver)? as usize,
            Size::UInt128(type_) => type_.from_stream(stream, _ver)? as usize,
            Size::Fixed(size)    => *size,
        })
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn to_bytes(&self, value: &Self::Type) -> std::io::Result<Vec<u8>> {
        match self {
            Size::UInt8(type_)   => type_.to_bytes(&(*value as u8)),
            Size::UInt16(type_)  => type_.to_bytes(&(*value as u16)),
            Size::UInt32(type_)  => type_.to_bytes(&(*value as u32)),
            Size::UInt64(type_)  => type_.to_bytes(&(*value as u64)),
            Size::UInt128(type_) => type_.to_bytes(&(*value as u128)),
            Size::Fixed(len)    => {
                if len != value {
                    Err(Error::new(ErrorKind::InvalidData, format!("Str/Array[{len}] given a string/list of length {value}. Help: For strings, this length is calculated AFTER encoding the string as bytes")))
                } else {
                    Ok(Vec::with_capacity(*len))
                }
            }
        }
    }
}