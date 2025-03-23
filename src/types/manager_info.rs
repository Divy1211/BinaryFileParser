use std::sync::{Arc, RwLock};

use pyo3::{pyclass, Bound, PyResult};
use pyo3::prelude::PyAnyMethods;

use crate::retrievers::retriever_ref::RetrieverRef;

#[pyclass(module = "bfp_rs")]
#[derive(Debug, Clone)]
pub struct ManagerInfo {
    refs: Arc<RwLock<Vec<RetrieverRef>>>,
}

impl ManagerInfo {
    pub fn new() -> Self {
        Self {
            refs: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn add_ref(&self, retriever: &Bound<RetrieverRef>) -> PyResult<()> {
        let ref_ = retriever.extract::<RetrieverRef>()?;
        let mut refs = self.refs.write().expect("GIL bound write");
        refs.push(ref_);
        Ok(())
    }
}
