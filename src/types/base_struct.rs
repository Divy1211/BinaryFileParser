use pyo3::prelude::*;
use pyo3::types::{PyType};
use pyo3::exceptions::PyTypeError;
use crate::retrievers::retriever::Retriever;
use crate::types::r#struct::Struct;
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
    fn new_py(ver: Version) -> Self {
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
        let struct_ = match cls.getattr("struct") {
            Ok(struct_) => struct_.downcast_into::<Struct>()?,
            Err(_) => {
                let struct_ = Bound::new(py, Struct::new())?;
                cls.setattr("struct", &struct_)?;
                struct_
            },
        };
        
        struct_.borrow().append(retriever)?;
        Ok(())
    }
}
