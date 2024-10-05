use std::sync::Arc;
use pyo3::{pyclass, PyObject};
use pyo3::prelude::*;
use pyo3::types::PyType;

use crate::errors::version_error::VersionError;
use crate::retrievers::map_validate::MapValidate;
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_type::{BfpType};
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs", extends = MapValidate)]
#[derive(Clone)]
pub struct Retriever {
    data_type: BfpType,

    min_ver: Version,
    max_ver: Version,

    default: Arc<PyObject>,
    default_factory: Arc<PyObject>,

    repeat: i32,
    remaining_compressed: bool,

    on_read: Arc<Vec<PyObject>>,
    on_write: Arc<Vec<PyObject>>,

    pub idx: usize,
}

#[pymethods]
impl Retriever {
    #[new]
    #[pyo3(signature = (data_type, min_ver = Version::new(vec!(-1)), max_ver = Version::new(vec!(1000)), default = None, default_factory = None, repeat = 1, remaining_compressed = false, on_read = None, on_write = None, mappers = None, validators = None, on_get = None, on_set = None))]
    fn new(
        py: Python,
        data_type: BfpType,

        min_ver: Version,
        max_ver: Version,

        default: Option<PyObject>,
        default_factory: Option<PyObject>,

        repeat: i32,
        remaining_compressed: bool,

        on_read: Option<Vec<PyObject>>,
        on_write: Option<Vec<PyObject>>,
        mappers: Option<Vec<PyObject>>,
        validators: Option<Vec<PyObject>>,
        on_get: Option<Vec<PyObject>>,
        on_set: Option<Vec<PyObject>>,
    ) -> PyResult<(Self, MapValidate)> {
        Ok((
            Retriever {
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
            },
            MapValidate::new(mappers, validators, on_get, on_set)
        ))
    }

    #[pyo3(name = "supported")]
    fn supported_py(&self, ver: &Version) -> bool {
        self.supported(ver)
    }

    fn __get__<'a, 'py>(
        slf: Bound<'py, Self>,
        instance: &'a Bound<'py, PyAny>,
        owner: &'a Bound<'py, PyType>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if instance.is_none() {
            return Ok(slf.into_any())
        }
        print!("hello from rust");
        let slf2 = slf.borrow();
        let instance2 = instance.downcast::<BaseStruct>()?.borrow();
        let super_ = slf2.as_ref();
        if !slf2.supported(&instance2.ver) {
            let name = &super_.name;
            let ver = &instance2.ver;
            return Err(VersionError::new_err(format!(
                "{name} is not supported in struct version {ver}"
            )))
        }
        let super_ = slf.into_any().downcast_into::<MapValidate>()?;
        MapValidate::__get__(super_, &instance.as_any(), owner)
    }

    fn __set__<'a, 'py>(
        slf: Bound<'py, Self>,
        instance: &'a Bound<'py, BaseStruct>,
        value: &'a Bound<'py, PyAny>,
    ) -> PyResult<()> {
        println!("test test");
        let slf2 = slf.borrow();
        let instance2 = instance.borrow();
        let super_ = slf2.as_ref();
        if !slf2.supported(&instance2.ver) {
            let name = &super_.name;
            let ver = &instance2.ver;
            return Err(VersionError::new_err(format!(
                "{name} is not supported in struct version {ver}"
            )))
        }

        let super_ = slf.into_any().downcast_into::<MapValidate>()?;
        MapValidate::__set__(super_, &instance.as_any(), value)
    }

    fn __set_name__(slf: Bound<Self>, owner: &Bound<PyType>, name: &str) -> PyResult<()> {
        let mut slf2 = slf.borrow_mut();
        let super_ = slf2.as_mut();
        super_.__set_name__(owner, name);
        drop(slf2);

        BaseStruct::_add_retriever(owner, &slf)?;

        Ok(())
    }

    fn secret_name(slf: PyRef<Self>) -> String {
        let name = &slf.as_ref().name;
        format!("_{name}")
    }
}

impl Retriever {
    #[cfg_attr(feature = "inline_always", inline(always))]
    pub fn supported(&self, ver: &Version) -> bool {
        self.min_ver <= *ver && *ver <= self.max_ver
    }

    pub fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> std::io::Result<ParseableType> {
        self.data_type.from_stream(stream, ver)
    }
}