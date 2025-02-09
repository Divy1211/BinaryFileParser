#[macro_export]
macro_rules! wrap_py {
    ($bfp_type:ty) => {
        #[pymethods]
        impl $bfp_type {
            #[pyo3(name = "to_bytes")]
            fn to_bytes_py(slf: PyRef<Self>, value: <Self as Parseable>::Type) -> PyResult<Bound<PyBytes>> {
                let bytes = slf.to_bytes(&value)?;
                Ok(PyBytes::new_bound(slf.py(), &bytes))
            }
        
            #[pyo3(name = "from_stream", signature = (stream, ver = Version::new(vec![0,])))]
            fn from_stream_py(slf: PyRef<Self>, stream: &mut ByteStream, ver: Version) -> PyResult<<Self as Parseable>::Type> {
                Ok(slf.from_stream(stream, &ver)?)
            }
        
            #[pyo3(name = "from_file")]
            fn from_file_py(slf: PyRef<Self>, filepath: &str) -> PyResult<<Self as Parseable>::Type> {
                Ok(slf.from_file(filepath)?)
            }
            #[pyo3(name = "from_bytes", signature = (bytes, ver = Version::new(vec![0,])))]
            fn from_bytes_py(slf: PyRef<Self>, bytes: &[u8], ver: Version) -> PyResult<<Self as Parseable>::Type> {
                Ok(slf.from_bytes(bytes, &ver)?)
            }
            #[pyo3(name = "to_file")]
            fn to_file_py(slf: PyRef<Self>, filepath: &str, value: <Self as Parseable>::Type) -> PyResult<()> {
                Ok(slf.to_file(filepath, &value)?)
            }
        }
    };
}