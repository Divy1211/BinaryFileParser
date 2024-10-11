use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

#[pyclass(module = "bfp_rs")]
#[derive(Debug, Clone)]
pub struct ByteStream {
    bytes: Arc<Vec<u8>>,
    progress: usize,
}

impl ByteStream {
    pub fn empty() -> Self {
        ByteStream {
            bytes: Arc::new(vec![]),
            progress: 0,
        }
    }
    
    pub fn from_file(filepath: &str) -> io::Result<Self> {
        let mut file = File::open(filepath)?;
        let mut bytes = Vec::new();

        file.read_to_end(&mut bytes)?;

        Ok(ByteStream {
            bytes: Arc::new(bytes),
            progress: 0,
        })
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        ByteStream {
            bytes: Arc::new(bytes.to_vec()),
            progress: 0,
        }
    }

    pub fn get(&mut self, n: usize) -> io::Result<&[u8]> {
        if n <= 0 {
            return Ok(&[]);
        }
        let len = self.bytes.len();
        if len < self.progress + n {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                format!("End of file reached (Requested {n} bytes, only {} left.)", len - self.progress)
            ));
        }

        let bytes = &self.bytes[self.progress..self.progress+n];
        self.progress += n;
        Ok(bytes)
    }

    pub fn peek(&self, n: usize) -> io::Result<&[u8]> {
        if n <= 0 {
            return Ok(&[]);
        }
        let len = self.bytes.len();
        if len < self.progress + n {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                format!("End of file reached (Requested {n} bytes, only {} left.)", len - self.progress)
            ));
        }

        let bytes = &self.bytes[self.progress..self.progress+n];
        Ok(bytes)
    }

    pub fn remaining(&mut self) -> &[u8] {
        let n = self.progress;
        self.progress = self.bytes.len();
        &self.bytes[n..]
    }
}

#[pymethods]
impl ByteStream {
    #[classmethod]
    #[pyo3(name = "from_file")]
    fn from_file_py(_cls: &Bound<PyType>, filepath: &str) -> PyResult<Self> {
        Ok(ByteStream::from_file(filepath)?)
    }

    #[classmethod]
    #[pyo3(name = "from_bytes")]
    fn from_bytes_py(_cls: &Bound<PyType>, bytes: &[u8]) -> Self {
        ByteStream::from_bytes(bytes)
    }

    #[pyo3(name = "get")]
    fn get_py<'py>(slf: Bound<'py, Self>, n: usize) -> PyResult<Bound<'py, PyBytes>> {
        let mut slf = slf.borrow_mut();
        let py = slf.py();
        let bytes = slf.get(n)?;
        Ok(PyBytes::new_bound(py, &bytes))
    }

    #[pyo3(name = "peek")]
    fn peek_py<'py>(slf: Bound<'py, Self>, n: usize) -> PyResult<Bound<'py, PyBytes>> {
        let slf = slf.borrow();
        let py = slf.py();
        let bytes = slf.peek(n)?;
        Ok(PyBytes::new_bound(py, &bytes))
    }

    #[pyo3(name = "remaining")]
    fn remaining_py<'py>(slf: Bound<'py, Self>) -> PyResult<Bound<'py, PyBytes>> {
        let mut slf = slf.borrow_mut();
        let py = slf.py();
        Ok(PyBytes::new_bound(py, slf.remaining()))
    }
}
