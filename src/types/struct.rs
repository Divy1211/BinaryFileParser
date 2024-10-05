use std::sync::{Arc, RwLock};

use pyo3::prelude::*;
use pyo3::types::PyType;
use crate::retrievers::retriever::Retriever;
use crate::types::bfp_type::BfpType;


#[pyclass(module = "bfp_rs")]
#[derive(Clone)]
pub struct Struct {
    pub retrievers: Arc<RwLock<Vec<Retriever>>>,
}

impl Struct {
    pub fn new() -> Self {
        Struct { retrievers: Arc::new(RwLock::new(Vec::with_capacity(1))) }
    }
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
