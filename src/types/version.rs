use std::fmt::{Display, Formatter};

use pyo3::prelude::*;
use pyo3::class::basic::CompareOp;
use pyo3::types::{PyTuple, PyType};

#[pyclass(module = "bfp_rs", frozen)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub struct Version {
    ver: Vec<i128>
}

#[pymethods]
impl Version {
    #[new]
    #[pyo3(signature = (*nums))]
    fn new_py(nums: &Bound<PyTuple>) -> PyResult<Self> {
        let ver = nums.iter()
            .map(|x| {x.extract()})
            .collect::<Result<Vec<i128>, _>>()?;

        Ok(Version { ver })
    }

    #[classmethod]
    fn from_str(_cls: &Bound<PyType>, ver_str: &str) -> PyResult<Self> {
        let ver = ver_str
            .split(".").into_iter()
            .map(|x| {x.parse()})
            .collect::<Result<Vec<i128>, _>>()?;

        Ok(Version { ver })
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{self}").to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        let nums = self.joined(", ");
        Ok(format!("Version({nums})").to_string())
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        op.matches(self.cmp(other))
    }
}

impl Version {
    pub fn new(ver: Vec<i128>) -> Self {
        Version { ver }
    }

    fn joined(&self, sep: &str) -> String {
        self.ver.iter()
            .map(|x| { x.to_string()})
            .collect::<Vec<_>>()
            .join(sep)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let nums = self.joined(".");
        write!(f, "v{nums}")
    }
}