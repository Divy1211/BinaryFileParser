use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyType;

#[pyclass(module = "bfp_rs", subclass)]
pub struct MapValidate {
    mappers: Vec<PyObject>,
    validators: Vec<PyObject>,
    on_get: Vec<PyObject>,
    on_set: Vec<PyObject>,

    pub name: String,
}

#[pymethods]
impl MapValidate {
    #[new]
    #[pyo3(signature = (mappers=None, validators=None, on_get=None, on_set=None))]
    pub fn new(
        mappers: Option<Vec<PyObject>>,
        validators: Option<Vec<PyObject>>,
        on_get: Option<Vec<PyObject>>,
        on_set: Option<Vec<PyObject>>,
    ) -> Self {
        MapValidate {
            mappers: mappers.unwrap_or_else(Vec::new),
            validators: validators.unwrap_or_else(Vec::new),
            on_get: on_get.unwrap_or_else(Vec::new),
            on_set: on_set.unwrap_or_else(Vec::new),

            name: String::from(""),
        }
    }

    pub fn __set_name__(&mut self, _owner: &Bound<PyType>, name: &str) {
        self.name = name.to_string()
    }

    pub fn __get__<'a, 'py>(
        slf: Bound<'py, Self>,
        instance: &'a Bound<'py, PyAny>,
        _owner: &'a Bound<'py, PyType>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = slf.py();
        if instance.is_none() {
            return Ok(slf.into_any())
        }
        let slf = slf.borrow();
        for func in &slf.on_get {
            func.call_bound(py, (&slf, instance,), None)?;
        }
        instance.getattr(slf.secret_name().as_str())
    }

    pub fn __set__<'a, 'py>(
        slf: Bound<'py, Self>,
        instance: &'a Bound<'py, PyAny>,
        value: &'a Bound<'py, PyAny>,
    ) -> PyResult<()> {
        let slf = slf.borrow();
        let py = instance.py();

        let mut val = value.to_object(py);
        for func in &slf.mappers {
            val = func.call_bound(py, (&slf, instance, &val), None)?;
        }

        for (i, func) in slf.validators.iter().enumerate() {
            let obj = func.call_bound(py, (&slf, instance, &val), None)?;
            if obj.is_truthy(py)? {
                continue
            }
            return Err(PyValueError::new_err(format!("Validator at index '{i}' failed")))
        }

        for func in &slf.on_set {
            func.call_bound(py, (&slf, instance,), None)?;
        }

        instance.setattr(slf.secret_name().as_str(), val)
    }
    fn secret_name(&self) -> String {
        let name = &self.name;
        format!("_{name}")
    }
}
