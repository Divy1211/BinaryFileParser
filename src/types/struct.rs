use std::sync::{Arc, RwLock};

use pyo3::prelude::*;
use pyo3::types::PyType;
use crate::retrievers::retriever::Retriever;
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs")]
#[derive(Clone)]
pub struct Struct {
    pub retrievers: Arc<RwLock<Vec<Retriever>>>,
    pub py_type: Arc<Py<PyType>>,
}

#[pymethods]
impl Struct {
    #[classmethod]
    fn __class_getitem__(_cls: &Bound<PyType>, base_struct_cls: &Bound<PyType>) -> PyResult<BfpType> {
        let struct_ = base_struct_cls
            .getattr("struct")?
            .extract::<Struct>()?;

        Ok(BfpType::Struct(struct_))
    }

    pub fn append(&self, retriever: &Bound<Retriever>) -> PyResult<usize> {
        let mut retriever = retriever.extract::<Retriever>()?;
        let mut retrievers = self.retrievers.write().unwrap(); // assert this is a GIL bound action
        let idx = retrievers.len();
        retriever.idx = idx;
        retrievers.push(retriever);
        Ok(idx)
    }
}

impl Struct {
    pub fn new(py_type: Py<PyType>) -> Self {
        Struct {
            retrievers: Arc::new(RwLock::new(Vec::with_capacity(1))),
            py_type: Arc::new(py_type),
        }
    }

    pub fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> std::io::Result<BaseStruct> {
        let retrievers = self.retrievers.read().unwrap(); // assert retrievers should never be modified after instantiation todo: change to Arc<Vec<>> and use a builder pattern
        let mut data = Vec::with_capacity(retrievers.len());
        for retriever in retrievers.iter() {
            if !retriever.supported(&ver) {
                data.push(None);
            }
            data.push(Some(retriever.from_stream(stream, &ver)?))
        }
        Ok(BaseStruct::new(ver.clone(), data))
    }

    pub fn to_bytes(&self, value: &BaseStruct) -> Vec<u8> {
        todo!()
    }
}
