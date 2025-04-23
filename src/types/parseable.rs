use std::fs::File;
use std::io::Write;
use pyo3::PyResult;
use crate::types::byte_stream::ByteStream;
use crate::types::version::Version;



// todo: figure out macros for fn delegations
pub trait Parseable {
    type Type;
    
    fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> PyResult<Self::Type>;

    fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()>;

    fn to_bytes(&self, value: &Self::Type) -> PyResult<Vec<u8>> {
        let mut buffer = Vec::new();
        self.to_bytes_in(value, &mut buffer)?;
        Ok(buffer)
    }
    
    fn from_bytes(&self, bytes: &[u8], ver: &Version) -> PyResult<Self::Type> {
        let mut stream = ByteStream::from_bytes(bytes);
        self.from_stream(&mut stream, ver)
    }

    fn from_file(&self, filepath: &str) -> PyResult<Self::Type> {
        let mut stream = ByteStream::from_file(filepath)?;
        Ok(self.from_stream(&mut stream, &Version::new(vec![0]))?)
    }
    
    fn to_file(&self, filepath: &str, value: &Self::Type) -> PyResult<()> {
        let bytes = self.to_bytes(value)?;
        let mut file = File::create(filepath)?;
        Ok(file.write_all(&bytes)?)
    }
}