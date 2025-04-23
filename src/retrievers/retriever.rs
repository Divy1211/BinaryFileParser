use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::{pyclass, PyObject};
use pyo3::exceptions::{PyValueError};
use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::errors::default_attribute_error::DefaultAttributeError;
use crate::errors::version_error::VersionError;
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_list::BfpList;
use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RetState {
    NoneList,
    NoneValue,
    Value,
    List,
}

#[pyclass(module = "bfp_rs")]
#[derive(Debug, Clone)]
pub struct Retriever {
    pub idx: usize,
    
    pub data_type: BfpType,

    min_ver: Version,
    max_ver: Version,

    pub repeat: isize,
    pub remaining_compressed: bool,

    pub name: String,
    
    on_read: Option<Arc<Vec<CombinatorType>>>,
    on_write: Option<Arc<Vec<CombinatorType>>>,
    
    default: Option<Arc<PyObject>>,
    default_factory: Option<Arc<PyObject>>,

    tmp_on_read: Option<Arc<PyObject>>,
    tmp_on_write: Option<Arc<PyObject>>,
}

#[pymethods]
impl Retriever {
    #[new]
    #[pyo3(signature = (
        data_type,
        *,
        min_ver = Version::new(vec![-1]), max_ver = Version::new(vec![10_000]),
        default = None, default_factory = None,
        repeat = 1,
        remaining_compressed = false,
        on_read = None, on_write = None
    ))]
    fn new(
        data_type: &Bound<PyAny>,

        min_ver: Version,
        max_ver: Version,

        default: Option<PyObject>,
        default_factory: Option<PyObject>,

        repeat: isize,
        remaining_compressed: bool,

        on_read: Option<PyObject>,
        on_write: Option<PyObject>,
    ) -> PyResult<Self> {
        let tmp_on_read = match on_read {
            None => { None }
            Some(obj) => { Some(Arc::new(obj)) }
        };

        let tmp_on_write = match on_write {
            None => { None }
            Some(obj) => { Some(Arc::new(obj)) }
        };
        
        if repeat < -2 {
            return Err(PyValueError::new_err("Repeat values cannot be less than -2"));
        }

        if repeat == -1 {
            return Err(PyValueError::new_err(
                "Repeat values should never be set to -1, as there is no way to dynamically indicate a single value with a set_repeat.\n\
                Note: If you're trying to make a value be parsed conditionally, use set_repeat to -1 instead"
            ));
        }
        
        Ok(Retriever {
            data_type: BfpType::from_py_any(data_type)?,
            min_ver,
            max_ver,
            default: default.map(Arc::new),
            default_factory: default_factory.map(Arc::new),
            repeat,
            remaining_compressed,
            on_read: None,
            on_write: None,
            tmp_on_read,
            tmp_on_write,
            idx: 0,
            name: String::new(),
        })
    }

    #[pyo3(name = "supported")]
    fn supported_py(&self, ver: &Version) -> bool {
        self.supported(ver)
    }

    fn __get__<'py>(
        slf: Bound<'py, Self>,
        instance: Bound<'py, PyAny>,
        _owner: Bound<'py, PyType>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if instance.is_none() {
            return Ok(slf.into_any())
        }
        let slf = slf.borrow();
        let instance = instance.downcast::<BaseStruct>()?.borrow();
        let inner = instance.inner();
        if !slf.supported(&inner.ver) {
            return Err(VersionError::new_err(format!(
                "'{}' is not supported in struct version {}", slf.name, inner.ver
            )))
        }

        Ok(
            inner.data[slf.idx].clone().expect("Attempting to access uninitialised data in struct")
                .to_bound(slf.py())
        )
    }

    fn __set__(
        slf: Bound<Self>,
        instance: Bound<BaseStruct>,
        value: Bound<PyAny>,
    ) -> PyResult<()> {
        if instance.is_none() {
            return Err(PyValueError::new_err("Retriever is not assignable"))
        }
        let slf = slf.borrow();
        let instance = instance.borrow();
        let mut inner = instance.inner_mut();
        if !slf.supported(&inner.ver) {
            return Err(VersionError::new_err(format!(
                "'{}' is not supported in struct version {}", slf.name, inner.ver
            )))
        }

        inner.data[slf.idx] = Some(match slf.state(&inner.repeats) {
            RetState::Value | RetState::NoneValue if value.is_none() => {
                inner.repeats[slf.idx] = Some(-1);
                ParseableType::None
            }
            RetState::Value | RetState::NoneValue => {
                inner.repeats[slf.idx] = None;
                slf.data_type.to_parseable(&value)?
            }
            RetState::List | RetState::NoneList if value.is_none() => {
                inner.repeats[slf.idx] = Some(-2);
                ParseableType::None
            }
            RetState::List | RetState::NoneList => {
                let repeat = slf.repeat(&inner.repeats);
                let len = value.len()? as isize;
                if repeat == -2 {
                    inner.repeats[slf.idx] = Some(len);
                } else if inner.repeats[slf.idx].is_none() && repeat != len {
                    return Err(PyValueError::new_err(format!(
                        "List length mismatch for '{}' which is a retriever of fixed repeat. Expected: {repeat}, Actual: {len}", slf.name
                    )))
                }
                let value = value.iter()?
                    .map(|v| {
                        slf.data_type.to_parseable(&v.expect("obtained from python"))
                    }).collect::<PyResult<Vec<_>>>()?;
                ParseableType::Array(BfpList::new(value, slf.data_type.clone()))
            }
        });
        Ok(())
    }

    fn __set_name__(slf: Bound<Self>, owner: &Bound<PyType>, name: &str) -> PyResult<()> {
        slf.borrow_mut().name = name.to_string();
        
        BaseStruct::add_ret(owner, &slf)?;

        Ok(())
    }
}

impl Retriever {
    pub fn from_default(&self, ver: &Version, repeats: &mut Vec<Option<isize>>, py: Python) -> PyResult<ParseableType> {
        let state = self.state(repeats);
        if state == RetState::NoneValue || state == RetState::NoneList {
            return Ok(ParseableType::None);
        }
        let repeat = self.repeat(repeats) as usize;

        if let Some(default) = self.default.as_ref() {
            let default = self.data_type.to_parseable(default.bind(py))?;
            if state == RetState::Value {
                return Ok(default);
            }
            let mut ls = Vec::with_capacity(repeat);
            for _ in 0..repeat {
                ls.push(default.clone());
            }
            return Ok(ParseableType::Array(BfpList::new(ls, self.data_type.clone())));
        }

        if let Some(default_factory) = self.default_factory.as_ref() {
            let first_default = default_factory
                .call_bound(py, (ver.clone(),), None)?
                .into_bound(py);
            if state == RetState::Value {
                if first_default.is_none() {
                    // let OptionX[T] consume the None first
                    if let Ok(default) = self.data_type.to_parseable(&first_default) {
                        return Ok(default);
                    }
                    repeats[self.idx] = Some(-1);
                    return Ok(ParseableType::None);
                }
                return self.data_type.to_parseable(&first_default);
            }
            if first_default.is_none() {
                repeats[self.idx] = Some(-2);
                return Ok(ParseableType::None);
            }

            let mut ls = Vec::with_capacity(repeat);

            if repeat > 0 {
                ls.push(self.data_type.to_parseable(&first_default)?);
            }

            for _ in 1..repeat {
                let default = default_factory
                    .call_bound(py, (ver.clone(),), None)?
                    .into_bound(py);
                ls.push(self.data_type.to_parseable(&default)?);
            }
            return Ok(ParseableType::Array(BfpList::new(ls, self.data_type.clone())));
        }

        Err(DefaultAttributeError::new_err(format!(
            "Unable to default initialise '{}' as a default value was not provided", self.name)
        ))
    }

    pub fn construct_fns(&mut self, py: Python) -> PyResult<()> {
        match &self.tmp_on_read {
            Some(obj) => {
                self.on_read = Some(Arc::new(obj.call0(py)?.extract::<Vec<CombinatorType>>(py)?));
                self.tmp_on_read = None;
            }
            _ => {}
        };

        match &self.tmp_on_write {
            Some(obj) => {
                self.on_write = Some(Arc::new(obj.call0(py)?.extract::<Vec<CombinatorType>>(py)?));
                self.tmp_on_write = None;
            }
            _ => {}
        };
        
        Ok(())
    }
    
    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn call_on_reads(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        let Some(on_read) = self.on_read.as_ref() else {
            return Ok(());
        };
        for combinator in on_read.iter() {
            combinator.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn call_on_writes(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        let Some(on_write) = self.on_write.as_ref() else {
            return Ok(());
        };
        for combinator in on_write.iter() {
            combinator.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn supported(&self, ver: &Version) -> bool {
        self.min_ver <= *ver && *ver <= self.max_ver
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> PyResult<ParseableType> {
        self.data_type.from_stream(stream, ver)
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn to_bytes_in(&self, value: &ParseableType, buffer: &mut Vec<u8>) -> PyResult<()> {
        self.data_type.to_bytes_in(value, buffer)
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn state(&self, repeats: &Vec<Option<isize>>) -> RetState {
        match repeats[self.idx] {
            Some(-2) => RetState::NoneList,
            Some(-1) => RetState::NoneValue,
            Some(_)  => RetState::List,
            None => {
                match self.repeat {
                    -2 => RetState::NoneList,
                    -1 => RetState::NoneValue,
                    1  => RetState::Value,
                    _  => RetState::List,
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