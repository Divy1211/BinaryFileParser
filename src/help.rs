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
        Self {
            ls: ls.clone(),
            old_state: *(ls.immutable.read().expect("GIL bound read"))
        }
    }
    
    pub fn __enter__(slf: PyRefMut<Self>) {
        *slf.ls.immutable.write().expect("Gil bound write") = false;
    }

    pub fn __exit__(
        slf: PyRefMut<Self>,
        _exc_type: PyObject,
        _exc_value: PyObject,
        _traceback: PyObject,
    ) -> PyResult<bool> {
        *slf.ls.immutable.write().expect("GIL bound write") = slf.old_state;
        Ok(false)
    }
}

#[pyfunction]
pub fn set_mut(ls: PyRefMut<BfpList>, value: bool) {
    *ls.immutable.write().expect("GIL bound write") = !value;
}