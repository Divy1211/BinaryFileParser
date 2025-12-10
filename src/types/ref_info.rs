use std::sync::{Arc, RwLock};

use pyo3::{pyclass, Bound, PyResult};
use pyo3::prelude::PyAnyMethods;

use crate::retrievers::retriever_ref::RetrieverRef;

#[pyclass(module = "bfp_rs")]
#[derive(Debug, Clone)]
pub struct RefInfo {
    refs: Arc<RwLock<Vec<RetrieverRef>>>,
}

impl Default for RefInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl RefInfo {
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
