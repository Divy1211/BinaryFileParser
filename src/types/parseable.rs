use std::fs::File;
use std::io;
use std::io::Write;
use crate::types::py_bfp_type::PyBfpType;
use crate::types::byte_stream::ByteStream;

pub trait Parseable {
    type Type;
    
    fn to_py_bfp_type() -> PyBfpType;
    
    fn from_stream(stream: &mut ByteStream) -> io::Result<Self::Type>;

    fn to_bytes(value: &Self::Type) -> Vec<u8>;

    fn from_bytes(bytes: &[u8]) -> io::Result<Self::Type> {
        let mut stream = ByteStream::from_bytes(bytes);
        Self::from_stream(&mut stream)
    }

    fn from_file(filepath: &str) -> io::Result<Self::Type> {
        let mut stream = ByteStream::from_file(filepath)?;
        Ok(Self::from_stream(&mut stream)?)
    }
    
    fn to_file(filepath: &str, value: &Self::Type) -> io::Result<()> {
        let bytes = Self::to_bytes(value);
        let mut file = File::create(filepath)?;
        Ok(file.write_all(&bytes)?)
    }
}