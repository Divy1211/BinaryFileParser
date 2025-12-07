use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::{make_struct, impl_into_pyobj, match_args_type};

#[pyclass(name = "Conflict", subclass)]
pub struct ConflictPy;

make_struct!(BasicPy(ConflictPy) as "Basic" {
    old: PyObject,
    change1: PyObject,
    change2: PyObject,
});

make_struct!(NestedConflictPy(ConflictPy) as "NestedConflict" {
    children: Py<PyDict>,
});
