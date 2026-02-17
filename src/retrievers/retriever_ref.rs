use std::cell::OnceCell;
use std::ffi::CString;
use std::sync::Arc;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple, PyType};
use crate::combinators::get::Get;
use crate::errors::version_error::VersionError;
use crate::retrievers::retriever::Retriever;
use crate::retrievers::retriever_combiner::RetrieverCombiner;
use crate::types::base_struct::BaseStruct;
use crate::types::ref_struct::RefStruct;

#[derive(Debug, Clone)]
enum Ref {
    Attr(String),
    Item(isize),
    Get(Get),
}


#[pyclass(module = "bfp_rs")]
#[derive(Debug, Clone)]
pub struct RetrieverRef  {
    target: Vec<Ref>,
    pub name: String,
    
    tuple: Option<Arc<Py<PyTuple>>>, // todo: Option this, so it can be none-ed after __set_name__ to lose the Arc
    enum_: Option<Arc<Py<PyType>>>,
}

#[pymethods]
impl RetrieverRef {
    #[new]
    #[pyo3(signature = (*target, r#enum = None), text_signature = "(*target: Retriever | RetrieverRef | RetrieverCombiner | int, enum = None)")]
    pub fn new(target: Bound<PyTuple>, r#enum: Option<Bound<PyType>>) -> PyResult<Self> {
        if <Bound<PyTuple> as PyTupleMethods>::len(&target) == 0 {
            return Err(PyValueError::new_err("RetrieverRef targets must contain at least one retriever"))
        }

        if target.get_item(0)?.extract::<usize>().is_ok() || target.get_item(0)?.extract::<Get>().is_ok() {
            return Err(PyValueError::new_err("RetrieverRef targets must begin with a retriever"))
        };

        let enum_ = match r#enum {
            None => None,
            Some(cls) => {
                let globals = PyDict::new(cls.py());
                cls.py().run(&CString::new("from enum import Enum")?, Some(&globals), None)?;
                let enum_cls = globals.get_item("Enum")?.expect("infallible");

                if !cls.is_subclass(&enum_cls)? {
                    return Err(PyTypeError::new_err(format!(
                        "Provided enum class '{}' does not subclass Enum", cls.fully_qualified_name()?
                    )));
                }

                Some(Arc::new(cls.unbind()))
            }
        };

        Ok(Self {
            target: Vec::new(),
            name: String::new(),
            tuple: Some(Arc::new(target.unbind())),
            enum_,
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
        let mut ref_struct = None;
        // Not checking for is_instance_of is fine, because refs can only be made in BaseStruct or RefStruct
        if let Ok(inner) = instance.getattr(intern!(slf.py(), "_struct")) {
            ref_struct = Some(instance);
            instance = inner;
        }
        let borrow = instance.cast::<BaseStruct>()?.borrow();
        let inner = borrow.inner();
        
        let target = &slf.borrow().target;
        let mut current = instance;
        let instance = OnceCell::new();
        
        for ref_ in target {
            let item = match ref_ {
                Ref::Attr(name) => current.getattr(name.as_str()),
                Ref::Item(idx) => {
                    let mut idx = *idx;
                    if idx < 0 {
                        idx += current.len()? as isize;
                    }
                    current.get_item(idx)
                },
                Ref::Get(get) => {
                    let Some(ref_struct) = &ref_struct else {
                        return Err(PyValueError::new_err("A get_attr can only be used in a RefStruct"));
                    };
                    let mut idx = get.eval_ref(ref_struct, instance.get().expect("Get is never first"))?;
                    if idx < 0 {
                        idx += current.len()? as i128;
                    }
                    current.get_item(idx)
                },
            }.map_err(|err| {
                if err.is_instance_of::<VersionError>(current.py()) {
                    return VersionError::new_err(format!(
                        "{} is not supported in struct version {}",
                        slf.borrow().name,
                        inner.ver,
                    ))
                }
                err
            })?;
            instance.get_or_init(|| current);
            current = item;
        }

        let this = slf.borrow();
        if let Some(cls) = &this.enum_ {
            let cls = cls.bind(slf.py());
            if let Ok(value) = cls.call1((&current,)) {
                current = value;
            }
        }
        Ok(current)
    }

    fn __set__<'py>(
        slf: Bound<'py, Self>,
        mut instance: Bound<'py, PyAny>,
        mut value: Bound<'py, PyAny>,
    ) -> PyResult<()> {
        let this = slf.borrow();
        if let Some(cls) = &this.enum_ {
            let cls = cls.bind(slf.py());
            if value.is_instance(cls)? {
                value = value.getattr(intern!(slf.py(), "value"))?;
            }
        }
        if instance.is_none() {
            return Err(PyValueError::new_err("RetrieverRef is not assignable"))
        }
        let mut ref_struct = None;
        // Not checking for is_instance_of is fine, because refs can only be made in BaseStruct or RefStruct
        if let Ok(inner) = instance.getattr(intern!(slf.py(), "_struct")) {
            ref_struct = Some(instance);
            instance = inner;
        }

        let borrow = instance.cast::<BaseStruct>()?.borrow();
        let inner = borrow.inner();
        let ver = inner.ver.clone();
        drop(inner);

        let target = &slf.borrow().target;
        let mut current = instance;
        let instance = OnceCell::new();

        for ref_ in &target[..target.len()-1] {
            let item = match ref_ {
                Ref::Attr(name) => current.getattr(name.as_str()),
                Ref::Item(idx) => {
                    let mut idx = *idx;
                    if idx < 0 {
                        idx += current.len()? as isize;
                    }
                    current.get_item(idx)
                },
                Ref::Get(get) => {
                    let Some(ref_struct) = &ref_struct else {
                        return Err(PyValueError::new_err("A get_attr can only be used in a RefStruct"));
                    };
                    let mut idx = get.eval_ref(ref_struct, instance.get().expect("Get is never first"))?;
                    if idx < 0 {
                        idx += current.len()? as i128;
                    }
                    current.get_item(idx)
                },
            }.map_err(|err| {
                if err.is_instance_of::<VersionError>(current.py()) {
                    return VersionError::new_err(format!(
                        "{} is not supported in struct version {}",
                        slf.borrow().name,
                        ver
                    ))
                }
                err
            })?;
            instance.get_or_init(|| current);
            current = item;
        }

        let Ok(()) = (match target.last().unwrap() {
            Ref::Attr(name) => current.setattr(name.as_str(), value),
            Ref::Item(idx) => current.set_item(*idx, value),
            Ref::Get(get) => {
                let Some(ref_struct) = &ref_struct else {
                    return Err(PyValueError::new_err("A get_attr can only be used in a RefStruct"));
                };
                current.set_item(get.eval_ref(ref_struct, instance.get().expect("Get is never first"))?, value)
            },
        }) else {
            return Err(VersionError::new_err(format!(
                "{} is not supported in struct version {}",
                slf.borrow().name,
                ver
            )))
        };
        Ok(())
    }
    
    fn __set_name__(slf: Bound<Self>, owner: &Bound<PyType>, name: &str) -> PyResult<()> {
        let mut this = slf.borrow_mut();
        this.name = name.to_string();

        this.target = this.tuple.take()
            .expect("This runs before none-d")
            .bind(slf.py())
            .into_iter().map(|val| {
            val.extract::<isize>()
                .map(Ref::Item)
                .or_else(|_err| val.cast::<Retriever>()
                    .map(|r| Ref::Attr(r.borrow().name.clone()))
                )
                .or_else(|_err| val.cast::<RetrieverRef>()
                    .map(|r| Ref::Attr(r.borrow().name.clone()))
                )
                .or_else(|_err| val.cast::<RetrieverCombiner>()
                    .map(|r| Ref::Attr(r.borrow().name.clone()))
                )
                .or_else(|_err| val.cast::<Get>()
                    .map_err(PyErr::from)
                    .and_then(|r| Ok(Ref::Get(r.extract()?))))
                .map_err(|_err| {
                    PyValueError::new_err("Ref targets must be retrievers, indexes, or get_attrs")
                })
        }).collect::<PyResult<_>>()?;
        drop(this);
        
        if owner.is_subclass_of::<RefStruct>()? {
            RefStruct::add_ref(owner, &slf)
        } else {
            BaseStruct::add_ref(owner, &slf)
        }
    }
}