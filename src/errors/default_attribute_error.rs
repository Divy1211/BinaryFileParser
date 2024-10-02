use pyo3::create_exception;
use pyo3::exceptions::PyAttributeError;

create_exception!(errors, DefaultAttributeError, PyAttributeError);
