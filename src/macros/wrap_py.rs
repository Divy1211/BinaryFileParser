#[macro_export]
macro_rules! wrap_py {
    ($bfp_type:ty) => {
        #[pymethods]
        impl $bfp_type {
            #[new]
            fn new_py() -> Self {
                Self {}
            }
            
            #[pyo3(name = "to_py_bfp_type")]
            #[staticmethod]
            fn to_py_bfp_type_py() -> PyResult<PyBfpType> {
                Ok(Self::to_py_bfp_type())
            }
            
            #[pyo3(name = "to_bytes")]
            #[staticmethod]
            fn to_bytes_py(py: Python, value: <Self as Parseable>::Type) -> PyResult<Bound<PyBytes>> {
                let bytes = Self::to_bytes(&value);
                Ok(PyBytes::new_bound(py, &bytes))
            }
        
            #[pyo3(name = "from_stream")]
            #[staticmethod]
            fn from_stream_py(stream: &mut ByteStream) -> PyResult<<Self as Parseable>::Type> {
                Ok(Self::from_stream(stream)?)
            }
        
            #[pyo3(name = "from_file")]
            #[staticmethod]
            fn from_file_py(filepath: &str) -> PyResult<<Self as Parseable>::Type> {
                Ok(Self::from_file(filepath)?)
            }
            #[pyo3(name = "from_bytes")]
            #[staticmethod]
            fn from_bytes_py(bytes: &[u8]) -> PyResult<<Self as Parseable>::Type> {
                Ok(Self::from_bytes(bytes)?)
            }
            #[pyo3(name = "to_file")]
            #[staticmethod]
            fn to_file_py(filepath: &str, value: <Self as Parseable>::Type) -> PyResult<()> {
                Ok(Self::to_file(filepath, &value)?)
            }
        }
    };
}