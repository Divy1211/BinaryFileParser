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
#[derive(Debug, Clone)]
pub struct BaseStruct {
    #[pyo3(get)]
    pub ver: Version,
    pub data: Arc<RwLock<Vec<Option<ParseableType>>>>,
}

impl BaseStruct {
    pub fn new(ver: Version, data: Vec<Option<ParseableType>>) -> Self {
        BaseStruct { ver, data: Arc::new(RwLock::new(data)) }
    }

    pub fn len(cls: &Bound<PyType>) -> PyResult<usize> {
        let struct_ = cls
            .getattr("struct")?
            .extract::<Struct>()?;

        let retrievers = struct_.retrievers.read().unwrap(); // assert retrievers should not be modified after instantiation
        Ok(retrievers.len())
    }
    
    pub fn with_cls<'py>(val: BaseStruct, cls: &Bound<'py, PyType>) -> PyResult<Bound<'py, PyAny>> {
        let obj = cls.call0()?;
        {
            let mut obj = obj.downcast::<BaseStruct>()?.borrow_mut();
            obj.ver = val.ver;
            obj.data = val.data;
        }
        Ok(obj)
    }
}

#[pymethods]
impl BaseStruct {
    #[new]
    #[classmethod]
    #[pyo3(signature = (ver = Version::new(vec!(-1))))]
    fn new_py(cls: &Bound<PyType>, ver: Version) -> PyResult<Self> {
        let vec = vec![None; BaseStruct::len(cls)?];
        Ok(BaseStruct { ver, data: Arc::new(RwLock::new(vec)) })
    }

    #[classmethod]
    pub fn _add_retriever(cls: &Bound<PyType>, retriever: &Bound<Retriever>) -> PyResult<()> {
        if !cls.is_subclass_of::<BaseStruct>()? {
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
    fn test_from_stream<'py>(cls: &Bound<'py, PyType>, stream: &mut ByteStream, ver: Version) -> PyResult<Bound<'py, PyAny>> {
        let struct_ = cls
            .getattr("struct").unwrap()// assert cls is BaseStruct subclass
            .downcast_into::<Struct>().unwrap() // assert BaseStruct subclasses have a "struct" attribute
            .borrow();

        let base = struct_.from_stream(stream, &ver)?;
        Ok(BaseStruct::with_cls(base, cls)?)
    }

    #[classmethod]
    #[pyo3(signature = (stream, ver = Version::new(vec![0,])))]
    fn from_stream(cls: &Bound<PyType>, stream: &mut ByteStream, ver: Version) -> io::Result<Self> {
        let struct_ = cls
            .getattr("struct").unwrap() // assert cls is BaseStruct subclass
            .downcast_into::<Struct>().unwrap() // assert BaseStruct subclasses have a "struct" attribute
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
