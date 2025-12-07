use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::{make_struct, impl_into_pyobj, match_args_type};

#[pyclass(name = "Diff", subclass)]
pub struct DiffPy;

make_struct!(InsertedPy(DiffPy) as "Inserted" {
    value: PyObject
});

make_struct!(DeletedPy(DiffPy) as "Deleted" {
    value: PyObject
});

make_struct!(ChangedPy(DiffPy) as "Changed" {
    old: PyObject,
    new: PyObject
});

make_struct!(NestedDiffPy(DiffPy) as "NestedDiff" {
    children: Py<PyDict>
});
