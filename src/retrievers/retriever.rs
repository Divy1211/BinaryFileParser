use std::sync::Arc;
use pyo3::{pyclass, PyObject};
use pyo3::prelude::*;
use pyo3::types::PyType;

use crate::errors::version_error::VersionError;
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_list::BfpList;
use crate::types::bfp_type::{BfpType};
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

pub enum RetState {
    None,
    Value,
    List,
}

#[pyclass(module = "bfp_rs")]
#[derive(Debug, Clone)]
pub struct Retriever {
    pub data_type: BfpType,

    min_ver: Version,
    max_ver: Version,

    default: Arc<PyObject>,
    default_factory: Arc<PyObject>,

    repeat: isize,
    remaining_compressed: bool,

    on_read: Arc<Vec<PyObject>>,
    on_write: Arc<Vec<PyObject>>,
    
    mappers: Arc<Vec<PyObject>>,
    validators: Arc<Vec<PyObject>>,
    
    name: String,
    pub idx: usize,
}

#[pymethods]
impl Retriever {
    #[new]
    #[pyo3(signature = (data_type, min_ver = Version::new(vec!(-1)), max_ver = Version::new(vec!(1000)), default = None, default_factory = None, repeat = 1, remaining_compressed = false, on_read = None, on_write = None, mappers = None, validators = None))]
    fn new(
        py: Python,
        data_type: BfpType,

        min_ver: Version,
        max_ver: Version,

        default: Option<PyObject>,
        default_factory: Option<PyObject>,

        repeat: isize,
        remaining_compressed: bool,

        on_read: Option<Vec<PyObject>>,
        on_write: Option<Vec<PyObject>>,
        mappers: Option<Vec<PyObject>>,
        validators: Option<Vec<PyObject>>,
    ) -> PyResult<Self> {
        Ok(Retriever {
            data_type,
            min_ver,
            max_ver,
            default: Arc::new(default.unwrap_or(py.None())),
            default_factory: Arc::new(default_factory.unwrap_or(py.None())),
            repeat,
            remaining_compressed,
            on_read: Arc::new(on_read.unwrap_or_else(Vec::new)),
            on_write: Arc::new(on_write.unwrap_or_else(Vec::new)),
            idx: 0,
            name: String::new(),
            mappers: Arc::new(mappers.unwrap_or_else(Vec::new)),
            validators: Arc::new(validators.unwrap_or_else(Vec::new)),
        })
    }

    #[pyo3(name = "supported")]
    fn supported_py(&self, ver: &Version) -> bool {
        self.supported(ver)
    }

    fn __get__<'a, 'py>(
        slf: Bound<'py, Retriever>,
        instance: &'a Bound<'py, BaseStruct>,
        _owner: &'a Bound<'py, PyType>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if instance.is_none() {
            return Ok(slf.into_any())
        }
        let slf = slf.borrow();
        let instance = instance.borrow();
        if !slf.supported(&instance.ver) {
            let ver = &instance.ver;
            return Err(VersionError::new_err(format!(
                "{} is not supported in struct version {ver}", slf.name
            )))
        }
        let data = instance.data.read().unwrap(); // assert this is a GIL bound action
        
        Ok(
            data[slf.idx].clone().unwrap() // assert this value should exist past the version check todo: default init breaks this assertion
                .to_bound(slf.py())
        )
    }

    fn __set__<'a, 'py>(
        slf: Bound<'py, Self>,
        instance: &'a Bound<'py, BaseStruct>,
        value: &'a Bound<'py, PyAny>,
    ) -> PyResult<()> {
        let slf = slf.borrow();
        let instance = instance.borrow();
        if !slf.supported(&instance.ver) {
            let ver = &instance.ver;
            return Err(VersionError::new_err(format!(
                "{} is not supported in struct version {ver}", slf.name
            )))
        }
        let repeats = instance.repeats.read().unwrap(); // assert this is a GIL bound action
        let mut data = instance.data.write().unwrap(); // assert this is a GIL bound action
        
        data[slf.idx] = Some(match slf.state(&repeats) {
            RetState::None => { ParseableType::None }
            RetState::Value => { slf.data_type.to_parseable(value)? }
            RetState::List => {
                let value = value.iter()?
                    .map(|v| {
                        slf.data_type.to_parseable(&v.unwrap())                       // assert v obtained from python
                    }).collect::<PyResult<Vec<_>>>()?;
                ParseableType::Array(BfpList::new(value, slf.data_type.clone()))
            }
        });
        Ok(())
    }

    fn __set_name__(slf: Bound<Self>, owner: &Bound<PyType>, name: &str) -> PyResult<()> {
        let mut slf2 = slf.borrow_mut();
        slf2.name = name.to_string();
        drop(slf2);

        BaseStruct::_add_retriever(owner, &slf)?;

        Ok(())
    }
}

impl Retriever {
    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn supported(&self, ver: &Version) -> bool {
        self.min_ver <= *ver && *ver <= self.max_ver
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> std::io::Result<ParseableType> {
        self.data_type.from_stream(stream, ver)
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn state(&self, repeats: &Vec<Option<isize>>) -> RetState {
        match repeats[self.idx] {
            Some(_) => { RetState::List }
            None => {
                match self.repeat {
                    -1 => { RetState::None },
                    1 => { RetState::Value },
                    _ => { RetState::List },
                }
            }
        }
    }
    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn repeat(&self, repeats: &Vec<Option<isize>>) -> isize {
        match repeats[self.idx] {
            Some(val) => { val }
            None => { self.repeat }
        }
    }
}