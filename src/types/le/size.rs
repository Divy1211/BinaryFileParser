use pyo3::exceptions::PyValueError;
use pyo3::PyResult;

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

impl Size {
    pub fn num_bytes(&self) -> usize {
        match self {
            Size::Fixed(_)   => 0,
            Size::UInt8(_)   => 1,
            Size::UInt16(_)  => 2,
            Size::UInt32(_)  => 4,
            Size::UInt64(_)  => 8,
            Size::UInt128(_) => 16,
        }
    }
    
    pub fn to_bytes_array(&self, value: usize) -> PyResult<[u8; 16]> {
        let mut out = [0; 16];
        match self {
            Size::UInt8(_) => {
                out[..1].copy_from_slice(&(value as u8).to_le_bytes())
            }
            Size::UInt16(_) => {
                out[..2].copy_from_slice(&(value as u16).to_le_bytes())
            }
            Size::UInt32(_) => {
                out[..4].copy_from_slice(&(value as u32).to_le_bytes())
            }
            Size::UInt64(_) => {
                out[..8].copy_from_slice(&(value as u64).to_le_bytes())
            }
            Size::UInt128(_) => {
                out[..16].copy_from_slice(&(value as u128).to_le_bytes())
            }
            Size::Fixed(len) => {
                if *len != value {
                    return Err(PyValueError::new_err(format!(
                        "Str/Array[{len}] given a string/list of length {value}. Help: For strings, this length is calculated AFTER encoding the string as bytes"
                    )));
                }
            }
        }
        Ok(out)
    }
}

impl Parseable for Size {
    type Type = usize;

    #[cfg_attr(feature = "inline_always", inline(always))]
    fn from_stream(&self, stream: &mut ByteStream, _ver: &Version) -> PyResult<Self::Type> {
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
    fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()> {
        match self {
            Size::UInt8(type_)   => type_.to_bytes_in(&(*value as u8), buffer),
            Size::UInt16(type_)  => type_.to_bytes_in(&(*value as u16), buffer),
            Size::UInt32(type_)  => type_.to_bytes_in(&(*value as u32), buffer),
            Size::UInt64(type_)  => type_.to_bytes_in(&(*value as u64), buffer),
            Size::UInt128(type_) => type_.to_bytes_in(&(*value as u128), buffer),
            Size::Fixed(len)    => {
                if len != value {
                    Err(PyValueError::new_err(format!(
                        "Str/Array[{len}] given a string/list of length {value}. Help: For strings, this length is calculated AFTER encoding the string as bytes"
                    )))
                } else {
                    Ok(())
                }
            }
        }
    }
}