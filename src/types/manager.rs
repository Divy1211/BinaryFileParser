use pyo3::prelude::*;
use pyo3::intern;
use pyo3::types::PyType;

use crate::retrievers::retriever_ref::RetrieverRef;
use crate::types::manager_info::ManagerInfo;

#[pyclass(module = "bfp_rs", subclass)]
#[derive(Debug)]
pub struct Manager {
    #[pyo3(get)]
    pub _struct: Py<PyAny>,
}

impl Manager {
    pub fn add_ref(cls: &Bound<PyType>, ref_: &Bound<RetrieverRef>) -> PyResult<()> {
        let info = match cls.getattr(intern!(cls.py(), "info")) {
            Ok(info) => info.downcast_into::<ManagerInfo>()?,
            Err(_) => {
                let info = Bound::new(cls.py(), ManagerInfo::new())?;
                cls.setattr("info", &info)?;
                info
            },
        }.borrow();

        info.add_ref(ref_)
    }
}

#[pymethods]
impl Manager {
    #[new]
    pub fn new_py(_struct: Py<PyAny>) -> Manager {
        Self { _struct }
    }
}