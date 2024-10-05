use std::sync::{Arc, RwLock};

use pyo3::prelude::*;
use pyo3::types::PyType;
use crate::retrievers::retriever::Retriever;
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs")]
#[derive(Clone)]
pub struct Struct {
    pub retrievers: Arc<RwLock<Vec<Retriever>>>,
}

#[pymethods]
impl Struct {
    #[classmethod]
    fn __class_getitem__(_cls: &Bound<'_, PyType>, base_struct_cls: &Bound<'_, PyType>) -> PyResult<BfpType> {
        let struct_ = base_struct_cls
            .getattr("struct")?
            .extract::<Struct>()?;

        Ok(BfpType::Struct(struct_))
    }

    pub fn append(&self, retriever: &Bound<Retriever>) -> PyResult<usize> {
        let mut retriever = retriever.extract::<Retriever>()?;
        let mut retrievers = self.retrievers.write().unwrap();
        let idx = retrievers.len();
        retriever.idx = idx;
        retrievers.push(retriever);
        Ok(idx)
    }
}

impl Struct {
    pub fn new() -> Self {
        Struct { retrievers: Arc::new(RwLock::new(Vec::with_capacity(1))) }
    }

    pub fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> std::io::Result<BaseStruct> {
        let retrievers = self.retrievers.read().unwrap();
        let mut data = Vec::with_capacity(retrievers.len());
        for retriever in retrievers.iter() {
            if !retriever.supported(&ver) {
                continue;
            }
            data.push(retriever.from_stream(stream, &ver)?)
        }
        
        Ok(BaseStruct::new(ver.clone(), data))
    }

    pub fn to_bytes(&self, value: &BaseStruct) -> Vec<u8> {
        todo!()
    }
}
