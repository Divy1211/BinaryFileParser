use std::sync::{Arc, RwLock};

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

use crate::errors::compression_error::CompressionError;
use crate::errors::version_error::VersionError;
use crate::retrievers::retriever::{RetState, Retriever};
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_list::BfpList;
use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs")]
#[derive(Debug, Clone)]
pub struct Struct {
    pub retrievers: Arc<RwLock<Vec<Retriever>>>,
    pub py_type: Arc<Py<PyType>>,

    pub get_ver: Option<Arc<PyObject>>,
    pub compress: Option<Arc<PyObject>>,
    pub decompress: Option<Arc<PyObject>>,
}

#[pymethods]
impl Struct {
    #[classmethod]
    fn __class_getitem__(_cls: &Bound<PyType>, sub_cls: &Bound<PyType>) -> PyResult<BfpType> {
        if !sub_cls.is_subclass_of::<BaseStruct>()? {
            return Err(PyTypeError::new_err(
                "Cannot create a BfpType from a class that does not subclass BaseStruct"
            ))
        }
        Ok(BfpType::Struct(Struct::from_cls(sub_cls)?))
    }
}

impl Struct {
    pub fn new(py_type: Py<PyType>) -> Self {
        Struct {
            retrievers: Arc::new(RwLock::new(Vec::with_capacity(1))),
            py_type: Arc::new(py_type),

            get_ver: None,
            compress: None,
            decompress: None,
        }
    }

    pub fn append(&self, retriever: &Bound<Retriever>) -> PyResult<usize> {
        let mut retriever = retriever.extract::<Retriever>()?;
        let mut retrievers = self.retrievers.write().expect("GIL bound write");
        let idx = retrievers.len();
        retriever.idx = idx;
        retrievers.push(retriever);
        Ok(idx)
    }

    pub fn from_cls(cls: &Bound<PyType>) -> PyResult<Self> {
        let mut struct_ = cls
            .getattr("struct").expect("always a BaseStruct subclass")
            .extract::<Struct>().expect("infallible");

        struct_.get_ver = get_if_impl(cls, "_get_version");
        struct_.compress = get_if_impl(cls, "_compress");
        struct_.decompress = get_if_impl(cls, "_decompress");

        Ok(struct_)
    }
    
    pub fn get_ver<'a>(&self, stream: &mut ByteStream, ver: &'a Version) -> PyResult<Version> {
        let Some(fn_) = &self.get_ver else {
            return Ok(ver.clone())
        };
        
        Python::with_gil(|py| {
            let ver = fn_.call_bound(py, (stream.clone(), ver.clone()), None)?;
            ver.extract::<Version>(py)
        })
    }

    pub fn decompress<'a>(&self, bytes: &[u8]) -> PyResult<ByteStream> {
        let Some(fn_) = &self.decompress else {
            return Err(CompressionError::new_err(
                "Unable to read object from file. A Structure with a compressed section needs to implement '_decompress' classmethod."
            ))
        };

        Python::with_gil(|py| {
            let bytes = fn_.call_bound(py, (PyBytes::new_bound(py, bytes),), None)?;
            Ok(ByteStream::from_bytes(bytes.extract::<&[u8]>(py)?))
        })
    }

    pub fn compress<'a>(&self, bytes: &[u8]) -> PyResult<Vec<u8>> {
        let Some(fn_) = &self.decompress else {
            return Err(CompressionError::new_err(
                "Unable to write object to file. A Structure with a compressed section needs to implement '_compress' classmethod."
            ))
        };

        Python::with_gil(|py| {
            let bytes = fn_.call_bound(py, (PyBytes::new_bound(py, bytes),), None)?;
            Ok(Vec::from(bytes.extract::<&[u8]>(py)?))
        })
    }
}

impl Parseable for Struct {
    type Type = BaseStruct;
    
    fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> std::io::Result<BaseStruct> {
        let retrievers = self.retrievers.read().expect("immutable"); // todo: change to Arc<Vec<>> with builder pattern?
        let mut data = Vec::with_capacity(retrievers.len());
        let repeats = vec![None; retrievers.len()];
        
        let ver = self.get_ver(stream, ver)?;
        
        for retriever in retrievers.iter() {
            if !retriever.supported(&ver) {
                data.push(None);
            }
            if retriever.remaining_compressed {
                *stream = self.decompress(stream.remaining())?
            }
            
            data.push(Some(match retriever.state(&repeats) {
                RetState::None => { ParseableType::None }
                RetState::Value => { retriever.from_stream(stream, &ver)? }
                RetState::List => {
                    let mut ls = Vec::with_capacity(retriever.repeat(&repeats) as usize);
                    for _ in 0..retriever.repeat(&repeats) {
                        ls.push(retriever.from_stream(stream, &ver)?);
                    }
                    BfpList::new(ls, retriever.data_type.clone()).into()
                }
            }))
        }
        Ok(BaseStruct::new(ver.clone(), data, repeats))
    }

    fn to_bytes(&self, _value: &BaseStruct) -> Vec<u8> {
        todo!()
    }
}


fn get_if_impl(cls: &Bound<PyType>, attr: &str) -> Option<Arc<PyObject>> {
    let py = cls.py();
    let obj = cls.getattr(attr).expect("always a BaseStruct subclass");
    if attr == "_get_version" {
        match obj.call1((ByteStream::empty(),)) {
            Err(err) if err.is_instance_of::<VersionError>(py) => None,
            _ => Some(Arc::new(obj.unbind()))
        }
    } else {
        match obj.call1((PyBytes::new_bound(py, &[]),)) {
            Err(err) if err.is_instance_of::<CompressionError>(py) => None,
            _ => Some(Arc::new(obj.unbind()))
        }
    }
}
