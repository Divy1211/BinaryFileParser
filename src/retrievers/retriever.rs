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
    pub data_type: BfpType,

    min_ver: Version,
    max_ver: Version,
    
    default: Arc<PyObject>,
    default_factory: Arc<PyObject>,
    
    repeat: isize,
    
    pub remaining_compressed: bool,
    
    pub on_read: Arc<Vec<CombinatorType>>,
    pub on_write: Arc<Vec<CombinatorType>>,

    tmp_on_read: Option<Arc<PyObject>>,
    tmp_on_write: Option<Arc<PyObject>>,
    
    pub name: String,
    pub idx: usize,
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
        py: Python,
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
            return Err(PyValueError::new_err("Repeat values cannot be less than -1"));
        }
        
        Ok(Retriever {
            data_type: BfpType::from_py_any(data_type)?,
            min_ver,
            max_ver,
            default: Arc::new(default.unwrap_or(py.None())),
            default_factory: Arc::new(default_factory.unwrap_or(py.None())),
            repeat,
            remaining_compressed,
            on_read: Arc::new(Vec::new()),
            on_write: Arc::new(Vec::new()),
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
        if !slf.supported(&instance.ver) {
            let ver = &instance.ver;
            return Err(VersionError::new_err(format!(
                "'{}' is not supported in struct version {ver}", slf.name
            )))
        }
        let data = instance.data.read().expect("GIL bound read");
        
        Ok(
            data[slf.idx].clone().expect("Attempting to access uninitialised data in struct")
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
        if !slf.supported(&instance.ver) {
            let ver = &instance.ver;
            return Err(VersionError::new_err(format!(
                "'{}' is not supported in struct version {ver}", slf.name
            )))
        }
        let mut repeats = instance.repeats.write().expect("GIL bound read");
        let mut data = instance.data.write().expect("GIL bound write");

        data[slf.idx] = Some(match slf.state(&repeats) {
            RetState::Value | RetState::NoneValue if value.is_none() => {
                repeats[slf.idx] = Some(-1);
                ParseableType::None
            }
            RetState::Value | RetState::NoneValue => {
                slf.data_type.to_parseable(&value)?
            }
            RetState::List | RetState::NoneList if value.is_none() => {
                repeats[slf.idx] = Some(-2);
                ParseableType::None
            }
            RetState::List | RetState::NoneList => {
                let repeat = slf.repeat(&repeats);
                let len = value.len()? as isize;
                if repeat == -2 {
                    repeats[slf.idx] = Some(len);
                } else if repeats[slf.idx].is_none() && repeat != len {
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
    pub fn from_default(&self, ver: &Version, repeats: &Vec<Option<isize>>, py: Python) -> PyResult<ParseableType> {
        let state = self.state(repeats);
        if state == RetState::NoneValue || state == RetState::NoneList {
            return Ok(ParseableType::None);
        }
        let repeat = self.repeat(repeats) as usize;

        if !self.default.is_none(py) {
            let default = self.data_type.to_parseable(self.default.bind(py));
            if state == RetState::Value {
                return default;
            }
            let default = default?;
            let mut ls = Vec::with_capacity(repeat);
            for _ in 0..repeat {
                ls.push(default.clone());
            }
            return Ok(ParseableType::Array(BfpList::new(ls, self.data_type.clone())));
        }

        if !self.default_factory.is_none(py) {
            if state == RetState::Value {
                return self.default_factory
                    .call_bound(py, (ver.clone(),), None) // default_factory(ver)
                    .and_then(|obj| self.data_type.to_parseable(obj.bind(py)));
            }
            let mut ls = Vec::with_capacity(repeat);
            for _ in 0..repeat {
                ls.push(
                    self.default_factory
                        .call_bound(py, (ver.clone(),), None) // default_factory(ver)
                        .and_then(|obj| self.data_type.to_parseable(obj.bind(py)))?
                );
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
                self.on_read = Arc::new(obj.call0(py)?.extract::<Vec<CombinatorType>>(py)?);
                self.tmp_on_read = None;
            }
            _ => {}
        };

        match &self.tmp_on_write {
            Some(obj) => {
                self.on_write = Arc::new(obj.call0(py)?.extract::<Vec<CombinatorType>>(py)?);
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
        for combinator in self.on_read.iter() {
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
        for combinator in self.on_write.iter() {
            combinator.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn supported(&self, ver: &Version) -> bool {
        self.min_ver <= *ver && *ver <= self.max_ver
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> std::io::Result<ParseableType> {
        self.data_type.from_stream(stream, ver)
    }

    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn to_bytes(&self, value: &ParseableType) -> std::io::Result<Vec<u8>> {
        if *value == ParseableType::None {
            return Ok(vec![]);
        }
        self.data_type.to_bytes(value)
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