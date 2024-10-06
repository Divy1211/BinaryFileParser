use std::fs::File;
use std::io;
use std::io::Write;
use std::sync::{Arc, RwLock};
use pyo3::prelude::*;
use pyo3::types::{PyType};
use pyo3::exceptions::PyTypeError;
use crate::retrievers::retriever::Retriever;
use crate::types::byte_stream::ByteStream;
use crate::types::parseable_type::ParseableType;
use crate::types::r#struct::Struct;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs", subclass)]
#[derive(Debug)]
pub struct BaseStruct {
    #[pyo3(get)]
    pub ver: Version,
    pub data: Arc<RwLock<Vec<ParseableType>>>,
}

impl BaseStruct {
    pub fn new(ver: Version, data: Vec<ParseableType>) -> Self {
        BaseStruct { ver, data: Arc::new(RwLock::new(data)) }
    }
}

#[pymethods]
impl BaseStruct {
    #[new]
    #[pyo3(signature = (ver = Version::new(vec!(-1))))]
    fn new_py(ver: Version) -> Self {
        BaseStruct { ver, data: Arc::new(RwLock::new(vec![])) }
    }
    
    #[classmethod]
    pub fn _add_retriever(cls: &Bound<PyType>, retriever: &Bound<Retriever>) -> PyResult<()> {
        if !cls.is_subclass_of::<BaseStruct>().unwrap() {
            return Err(PyTypeError::new_err(
                "Cannot create retrievers in classes that do not subclass BaseStruct"
            ))
        }
        let py = cls.py();
        let struct_ = match cls.getattr("struct") {
            Ok(struct_) => struct_.downcast_into::<Struct>()?,
            Err(_) => {
                let struct_ = Bound::new(py, Struct::new(cls.extract()?))?;
                cls.setattr("struct", &struct_)?;
                struct_
            },
        }.borrow();
        let idx = struct_.append(retriever)?;
        retriever.borrow_mut().idx = idx;
        Ok(())
    }
    
    #[classmethod]
    #[pyo3(signature = (stream, ver = Version::new(vec![0,])))]
    fn from_stream(cls: &Bound<PyType>, stream: &mut ByteStream, ver: Version) -> io::Result<Self> {
        let struct_ = cls
            .getattr("struct").unwrap()
            .downcast_into::<Struct>().unwrap()
            .borrow();
        struct_.from_stream(stream, &ver)
    }

    fn to_bytes(&self, value: &BaseStruct) -> Vec<u8> {
        todo!()
    }

    #[classmethod]
    fn from_bytes(cls: &Bound<PyType>, bytes: &[u8]) -> io::Result<BaseStruct> {
        let mut stream = ByteStream::from_bytes(bytes);
        BaseStruct::from_stream(cls, &mut stream, Version::new(vec![0, ]))
    }

    #[classmethod]
    fn from_file(cls: &Bound<PyType>, filepath: &str) -> io::Result<BaseStruct> {
        let mut stream = ByteStream::from_file(filepath)?;
        BaseStruct::from_stream(cls, &mut stream, Version::new(vec![0, ]))
    }

    fn to_file(&self, filepath: &str, value: &BaseStruct) -> io::Result<()> {
        let bytes = self.to_bytes(value);
        let mut file = File::create(filepath)?;
        Ok(file.write_all(&bytes)?)
    }
}
