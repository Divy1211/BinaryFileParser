#[macro_export]
macro_rules! wrap_py {
    ($bfp_type:ty) => {
        #[pymethods]
        impl $bfp_type {
            #[new]
            fn new_py() -> Self {
                Self {}
            }
        
            #[pyo3(name = "to_bytes")]
            fn to_bytes_py(slf: PyRef<Self>, value: <Self as Parseable>::Type) -> PyResult<Bound<PyBytes>> {
                let bytes = slf.to_bytes(&value);
                Ok(PyBytes::new_bound(slf.py(), &bytes))
            }
        
            #[pyo3(name = "from_stream")]
            fn from_stream_py(slf: PyRef<Self>, stream: &mut ByteStream) -> PyResult<<Self as Parseable>::Type> {
                Ok(slf.from_stream(stream)?)
            }
        
            #[pyo3(name = "from_file")]
            fn from_file_py(slf: PyRef<Self>, filepath: &str) -> PyResult<<Self as Parseable>::Type> {
                Ok(slf.from_file(filepath)?)
            }
            #[pyo3(name = "from_bytes")]
            fn from_bytes_py(slf: PyRef<Self>, bytes: &[u8]) -> PyResult<<Self as Parseable>::Type> {
                Ok(slf.from_bytes(bytes)?)
            }
            #[pyo3(name = "to_file")]
            fn to_file_py(slf: PyRef<Self>, filepath: &str, value: <Self as Parseable>::Type) -> PyResult<()> {
                Ok(slf.to_file(filepath, &value)?)
            }
        }
    };
}