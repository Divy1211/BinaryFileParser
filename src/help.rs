use pyo3::prelude::*;

use crate::types::bfp_list::BfpList;

#[pyclass(module = "bfp_rs", name = "borrow_mut")]
#[derive(Debug)]
pub struct BorrowMutGuard {
    ls: BfpList,
    old_state: bool,
}

#[pymethods]
impl BorrowMutGuard {
    #[new]
    pub fn new_py(ls: PyRef<BfpList>) -> BorrowMutGuard {
        let inner = ls.inner();
        Self {
            ls: ls.clone(),
            old_state: inner.immutable
        }
    }
    
    pub fn __enter__(slf: PyRefMut<Self>) {
        let mut inner = slf.ls.inner_mut();
        inner.immutable = false;
    }

    pub fn __exit__(
        slf: PyRefMut<Self>,
        _exc_type: PyObject,
        _exc_value: PyObject,
        _traceback: PyObject,
    ) -> PyResult<bool> {
        let mut inner = slf.ls.inner_mut();
        inner.immutable = slf.old_state;
        Ok(false)
    }
}

#[pyfunction]
pub fn set_mut(ls: PyRefMut<BfpList>, value: bool) {
    let mut inner = ls.inner_mut();
    inner.immutable = !value;
}