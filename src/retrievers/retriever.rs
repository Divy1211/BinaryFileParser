use pyo3::{pyclass, PyObject};
use pyo3::prelude::*;
use pyo3::types::PyType;

use crate::errors::version_error::VersionError;
use crate::retrievers::map_validate::MapValidate;
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_type::{BfpType};
use crate::types::le::int16::Int16;
use crate::types::le::int8::Int8;
use crate::types::py_bfp_type::PyBfpType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs", extends = MapValidate)]
pub struct Retriever {
    data_type: BfpType,

    min_ver: Version,
    max_ver: Version,

    default: PyObject,
    default_factory: PyObject,

    repeat: i32,
    remaining_compressed: bool,

    on_read: Vec<PyObject>,
    on_write: Vec<PyObject>,
}

#[pymethods]
impl Retriever {
    #[new]
    #[pyo3(signature = (data_type, min_ver = Version::new(vec!(-1)), max_ver = Version::new(vec!(1000)), default = None, default_factory = None, repeat = 1, remaining_compressed = false, on_read = None, on_write = None, mappers = None, validators = None, on_get = None, on_set = None))]
    fn new(
        py: Python,
        data_type: PyObject,

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
        let py_bfp_type = data_type
            .getattr(py, "to_py_bfp_type")?
            .call_bound(py, (), None)?
            .extract::<PyBfpType>(py)?;
        
        let data_type = match py_bfp_type {
            PyBfpType::Int8 => data_type.extract::<Int8>(py)?.into(),
            PyBfpType::Int16 => data_type.extract::<Int16>(py)?.into(),
            _ => { todo!() }
        };
        
        Ok((
            Retriever {
                data_type,
                min_ver,
                max_ver,
                default: default.unwrap_or(py.None()),
                default_factory: default_factory.unwrap_or(py.None()),
                repeat,
                remaining_compressed,
                on_read: on_read.unwrap_or_else(Vec::new),
                on_write: on_write.unwrap_or_else(Vec::new),
            },
            MapValidate::new(mappers, validators, on_get, on_set)
        ))
    }

    fn supported(&self, ver: &Version) -> bool {
        self.min_ver <= *ver && *ver <= self.max_ver
    }

    fn __get__<'a, 'py>(
        slf: Bound<'py, Self>,
        instance: &'a Bound<'py, PyAny>,
        owner: &'a Bound<'py, PyType>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if instance.is_none() {
            return Ok(slf.into_any())
        }
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

        BaseStruct::_add_retriever(owner, &slf)?;

        Ok(())
    }

    fn secret_name(slf: PyRef<Self>) -> String {
        let name = &slf.as_ref().name;
        format!("_{name}")
    }
}
