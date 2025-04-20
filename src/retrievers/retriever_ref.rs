use std::sync::Arc;
use pyo3::exceptions::PyValueError;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyType};
use crate::errors::version_error::VersionError;
use crate::retrievers::retriever::Retriever;
use crate::retrievers::retriever_combiner::RetrieverCombiner;
use crate::types::base_struct::BaseStruct;
use crate::types::manager::Manager;

#[derive(Debug, Clone)]
enum Ref {
    Attr(String),
    Item(usize),
}


#[pyclass(module = "bfp_rs")]
#[derive(Debug, Clone)]
pub struct RetrieverRef  {
    target: Vec<Ref>,
    pub name: String,
    
    tuple: Arc<Py<PyTuple>>, // todo: Option this, so it can be none-ed after __set_name__ to lose the Arc
}

#[pymethods]
impl RetrieverRef {
    #[new]
    #[pyo3(signature = (*target), text_signature = "(*target: Retriever | RetrieverRef | RetrieverCombiner | int)")]
    pub fn new(target: Bound<PyTuple>) -> PyResult<Self> {
        if <Bound<PyTuple> as PyTupleMethods>::len(&target) == 0 {
            return Err(PyValueError::new_err("Ref targets must contain at least one retriever"))
        }

        if let Ok(_fst) = target.get_item(0)?.extract::<usize>() {
            return Err(PyValueError::new_err("Ref targets must begin with a retriever"))
        };
        
        Ok(Self {
            target: Vec::new(),
            name: String::new(),
            tuple: Arc::new(target.unbind()),
        })
    }

    fn __get__<'py>(
        slf: Bound<'py, Self>,
        mut instance: Bound<'py, PyAny>,
        _owner: Bound<'py, PyType>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if instance.is_none() {
            return Ok(slf.into_any())
        }
        // Not checking for is_instance_of is fine, because refs can only be made in BaseStruct or Manager
        if let Ok(inner) = instance.getattr(intern!(slf.py(), "_struct")) {
            instance = inner;
        }
        let borrow = instance.downcast::<BaseStruct>()?.borrow();
        let inner = borrow.inner();
        
        let target = &slf.borrow().target;
        let mut current = instance;
        
        for ref_ in target {
            let Ok(item) = (match ref_ {
                Ref::Attr(name) => current.getattr(name.as_str()),
                Ref::Item(idx) => current.get_item(*idx),
            }) else {
                return Err(VersionError::new_err(format!(
                    "{} is not supported in struct version {}",
                    slf.borrow().name,
                    inner.ver,
                )))
            };
            current = item;
        }
        
        Ok(current)
    }

    fn __set__(
        slf: Bound<Self>,
        mut instance: Bound<PyAny>,
        value: Bound<PyAny>,
    ) -> PyResult<()> {
        if instance.is_none() {
            return Err(PyValueError::new_err("RetrieverRef is not assignable"))
        }
        // Not checking for is_instance_of is fine, because refs can only be made in BaseStruct or Manager
        if let Ok(inner) = instance.getattr(intern!(slf.py(), "_struct")) {
            instance = inner;
        }

        let borrow = instance.downcast::<BaseStruct>()?.borrow();
        let inner = borrow.inner();

        let target = &slf.borrow().target;
        let mut current = instance;

        for ref_ in &target[..target.len()-1] {
            let Ok(item) = (match ref_ {
                Ref::Attr(name) => current.getattr(name.as_str()),
                Ref::Item(idx) => current.get_item(*idx),
            }) else {
                return Err(VersionError::new_err(format!(
                    "{} is not supported in struct version {}",
                    slf.borrow().name,
                    inner.ver
                )))
            };
            current = item;
        }

        let Ok(()) = (match target.last().unwrap() {
            Ref::Attr(name) => current.setattr(name.as_str(), value),
            Ref::Item(idx) => current.set_item(*idx, value),
        }) else {
            return Err(VersionError::new_err(format!(
                "{} is not supported in struct version {}",
                slf.borrow().name,
                inner.ver
            )))
        };
        Ok(())
    }
    
    fn __set_name__(slf: Bound<Self>, owner: &Bound<PyType>, name: &str) -> PyResult<()> {
        let mut this = slf.borrow_mut();
        this.name = name.to_string();

        this.target = this.tuple.bind(slf.py()).into_iter().map(|val| {
            val.extract::<usize>()
                .map(|num| Ref::Item(num))
                .or_else(|_err| val.downcast::<Retriever>().map(|r| Ref::Attr(r.borrow().name.clone())))
                .or_else(|_err| val.downcast::<RetrieverRef>().map(|r| Ref::Attr(r.borrow().name.clone())))
                .or_else(|_err| val.downcast::<RetrieverCombiner>().map(|r| Ref::Attr(r.borrow().name.clone())))
                .map_err(|_err| {
                    PyValueError::new_err("Ref targets must be retrievers or indexes")
                })
        }).collect::<PyResult<_>>()?;
        drop(this);
        
        if owner.is_subclass_of::<Manager>()? {
            Manager::add_ref(owner, &slf)
        } else {
            BaseStruct::add_ref(owner, &slf)
        }
    }
}