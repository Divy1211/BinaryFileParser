use pyo3::exceptions::{PyTypeError};
use pyo3::prelude::*;
use pyo3::intern;
use pyo3::types::PyType;

use crate::retrievers::retriever_ref::RetrieverRef;
use crate::types::base_struct::BaseStruct;
use crate::types::ref_info::RefInfo;

#[pyclass(module = "bfp_rs", subclass)]
#[derive(Debug)]
pub struct RefStruct {
    #[pyo3(get)]
    pub _struct: Py<PyAny>,
}

impl RefStruct {
    pub fn add_ref(cls: &Bound<PyType>, ref_: &Bound<RetrieverRef>) -> PyResult<()> {
        let info = match cls.getattr(intern!(cls.py(), "info")) {
            Ok(info) => info.cast_into::<RefInfo>()?,
            Err(_) => {
                let info = Bound::new(cls.py(), RefInfo::new())?;
                cls.setattr("info", &info)?;
                info
            },
        }.borrow();

        info.add_ref(ref_)
    }
}

#[pymethods]
impl RefStruct {
    #[new]
    pub fn new_py(_struct: Py<PyAny>, py: Python) -> PyResult<RefStruct> {
        if !_struct.bind(py).is_instance_of::<BaseStruct>() {
            return Err(PyTypeError::new_err("RefStructs must hold base struct instances only"))
        }
        Ok(Self { _struct })
    }
}