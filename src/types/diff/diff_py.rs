use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::{make_struct, impl_into_pyobj, match_args_type};

#[pyclass(name = "Diff", subclass)]
pub struct DiffPy;

make_struct!(InsertedPy(DiffPy) as "Inserted" {
    value: PyObject
} impl {});

make_struct!(DeletedPy(DiffPy) as "Deleted" {
    value: PyObject
} impl {});

make_struct!(ChangedPy(DiffPy) as "Changed" {
    old: PyObject,
    new: PyObject
} impl {});

make_struct!(NestedDiffPy(DiffPy) as "NestedDiff" {
    children: Py<PyDict>
} impl {
    fn __getitem__<'py>(slf: Bound<'py, Self>, key: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.borrow();
        PyAnyMethods::get_item(slf.children.as_any().bind(slf.py()), key)
    }
});
