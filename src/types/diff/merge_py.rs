use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::{make_struct, impl_into_pyobj, match_args_type};

#[pyclass(name = "Conflict", subclass)]
pub struct ConflictPy;

make_struct!(BasicPy(ConflictPy) as "Basic" {
    old: Option<PyObject>,
    change1: PyObject,
    change2: PyObject,
} impl {});

make_struct!(NestedConflictPy(ConflictPy) as "NestedConflict" {
    children: Py<PyDict>,
} impl {
    fn __getitem__<'py>(slf: Bound<'py, Self>, key: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.borrow();
        PyAnyMethods::get_item(slf.children.as_any().bind(slf.py()), key)
    }
});
