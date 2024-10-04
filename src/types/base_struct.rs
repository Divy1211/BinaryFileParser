use pyo3::prelude::*;
use pyo3::types::{PyList, PyType};
use pyo3::{PyTypeInfo};
use pyo3::exceptions::PyTypeError;
use crate::retrievers::retriever::Retriever;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs", subclass)]
pub struct BaseStruct {
    #[pyo3(get)]
    pub ver: Version,
}

#[pymethods]
impl BaseStruct {
    #[new]
    #[pyo3(signature = (ver = Version::new(vec!(-1))))]
    fn new(ver: Version) -> Self {
        BaseStruct { ver }
    }

    #[classmethod]
    pub fn _add_retriever(cls: &Bound<PyType>, retriever: &Bound<Retriever>) -> PyResult<()> {
        if !cls.is_subclass_of::<BaseStruct>().unwrap() {
            return Err(PyTypeError::new_err(
                "Cannot create retrievers in classes that do not subclass BaseStruct"
            ))
        }
        let py = cls.py();
        let ls = match cls.getattr("retrievers") {
            Ok(ls) => ls,
            Err(_) => {
                let ls = PyList::empty_bound(py).into_any();
                cls.setattr("retrievers", &ls)?;
                ls
            },
        };

        let ls = ls.downcast::<PyList>()?;

        ls.append(retriever)?;

        Ok(())
    }
}
