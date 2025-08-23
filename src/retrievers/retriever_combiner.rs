use std::sync::Arc;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyType};
use crate::errors::version_error::VersionError;
use crate::retrievers::retriever::Retriever;
use crate::retrievers::retriever_ref::RetrieverRef;
use crate::types::base_struct::BaseStruct;

#[pyclass(module = "bfp_rs")]
#[derive(Debug, Clone)]
pub struct RetrieverCombiner  {
    target: Vec<String>,
    pub name: String,
    
    tuple: Arc<Py<PyTuple>> // todo: Option this, so it can be none-ed after __set_name__ to lose the Arc
}

#[pymethods]
impl RetrieverCombiner {
    #[new]
    #[pyo3(signature = (*target), text_signature = "(*target: Retriever | RetrieverRef | RetrieverCombiner)")]
    pub fn new(target: Bound<PyTuple>) -> PyResult<Self> {
        if <Bound<PyTuple> as PyTupleMethods>::len(&target) == 0 {
            return Err(PyValueError::new_err("Combiner targets must contain at least one retriever"))
        }

        Ok(Self {
            target: Vec::new(),
            name: String::new(),
            tuple: Arc::new(target.unbind())
        })
    }

    fn __get__<'py>(
        slf: Bound<'py, Self>,
        instance: Bound<'py, PyAny>,
        _owner: Bound<'py, PyType>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if instance.is_none() {
            return Ok(slf.into_any())
        }

        let target = &slf.borrow().target;

        for attr in target {
            if let Ok(val) = instance.getattr(attr.as_str()) {
                return Ok(val);
            };
        }
        Err(VersionError::new_err(format!(
            "{} is not supported in struct version {}",
            slf.borrow().name,
            instance.downcast::<BaseStruct>()?.borrow().inner().ver
        )))
    }

    fn __set__(
        slf: Bound<Self>,
        instance: Bound<PyAny>,
        value: &Bound<PyAny>,
    ) -> PyResult<()> {
        if instance.is_none() {
            return Err(PyValueError::new_err("RetrieverCombiner is not assignable"))
        }

        let target = &slf.borrow().target;

        for attr in target {
            if let Ok(()) = instance.setattr(attr.as_str(), value) {
                return Ok(());
            };
        }
        Err(VersionError::new_err(format!(
            "{} is not supported in struct version {}",
            slf.borrow().name,
            instance.downcast::<BaseStruct>()?.borrow().inner().ver
        )))
    }

    fn __set_name__(slf: Bound<Self>, owner: &Bound<PyType>, name: &str) -> PyResult<()> {
        let mut this = slf.borrow_mut();
        this.name = name.to_string();

        this.target = this.tuple.bind(slf.py()).into_iter().map(|val| {
            val.downcast::<Retriever>()
                .map(|r| r.borrow().name.clone())
                .or_else(|_err| val.downcast::<RetrieverRef>().map(|r| r.borrow().name.clone()))
                .or_else(|_err| val.downcast::<RetrieverCombiner>().map(|r| r.borrow().name.clone()))
                .map_err(|_err| {
                    PyValueError::new_err("Combiner targets must be retrievers")
                })
        }).collect::<PyResult<_>>()?;
        drop(this);

        BaseStruct::add_comb(owner, &slf)?;

        Ok(())
    }
}