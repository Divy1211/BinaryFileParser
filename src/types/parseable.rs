use std::fs::File;
use std::io;
use std::io::Write;

use crate::types::byte_stream::ByteStream;
use crate::types::version::Version;

pub trait Parseable {
    type Type;
    
    fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> io::Result<Self::Type>;

    fn to_bytes(&self, value: &Self::Type) -> io::Result<Vec<u8>>;

    fn from_bytes(&self, bytes: &[u8], ver: &Version) -> io::Result<Self::Type> {
        let mut stream = ByteStream::from_bytes(bytes);
        self.from_stream(&mut stream, ver)
    }

    fn from_file(&self, filepath: &str) -> io::Result<Self::Type> {
        let mut stream = ByteStream::from_file(filepath)?;
        Ok(self.from_stream(&mut stream, &Version::new(vec![0]))?)
    }
    
    fn to_file(&self, filepath: &str, value: &Self::Type) -> io::Result<()> {
        let bytes = self.to_bytes(value)?;
        let mut file = File::create(filepath)?;
        Ok(file.write_all(&bytes)?)
    }
}