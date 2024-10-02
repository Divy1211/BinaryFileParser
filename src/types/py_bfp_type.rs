use pyo3::pyclass;

#[pyclass(module = "bfp_rs", eq, eq_int)]
#[derive(PartialEq, Eq, Clone)]
pub enum PyBfpType {
    Int8,
    Int16,
}