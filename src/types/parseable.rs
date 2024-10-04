use std::fs::File;
use std::io;
use std::io::Write;
use crate::types::byte_stream::ByteStream;

pub trait Parseable {
    type Type;
    
    fn from_stream(&self, stream: &mut ByteStream) -> io::Result<Self::Type>;

    fn to_bytes(&self, value: &Self::Type) -> Vec<u8>;

    fn from_bytes(&self, bytes: &[u8]) -> io::Result<Self::Type> {
        let mut stream = ByteStream::from_bytes(bytes);
        self.from_stream(&mut stream)
    }

    fn from_file(&self, filepath: &str) -> io::Result<Self::Type> {
        let mut stream = ByteStream::from_file(filepath)?;
        Ok(self.from_stream(&mut stream)?)
    }
    
    fn to_file(&self, filepath: &str, value: &Self::Type) -> io::Result<()> {
        let bytes = self.to_bytes(value);
        let mut file = File::create(filepath)?;
        Ok(file.write_all(&bytes)?)
    }
}