use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::{make_struct, impl_into_pyobj, match_args_type};

#[pyclass(name = "Conflict", subclass)]
pub struct ConflictPy;

make_struct!(BasicPy(ConflictPy) as "Basic" {
    old: Option<Py<PyAny>>,
    change1: Py<PyAny>,
    change2: Py<PyAny>,
} impl {
    fn __repr__(&self) -> String {
        format!(
            "Basic(old: {}, change1: {}, change2: {})",
            self.old.as_ref().map(|value| value.to_string()).unwrap_or(String::from("None")),
            self.change1,
            self.change2,
        )
    }
});

make_struct!(NestedConflictPy(ConflictPy) as "NestedConflict" {
    children: Py<PyDict>,
} impl {
    fn __getitem__<'py>(slf: Bound<'py, Self>, key: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.borrow();
        PyAnyMethods::get_item(slf.children.as_any().bind(slf.py()), key)
    }
    
    fn __repr__(&self) -> String {
        format!("NestedConflict(children: {})", self.children)
    }
});
