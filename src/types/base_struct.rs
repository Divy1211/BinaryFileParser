use std::fs::File;
use std::io::Write;
use std::sync::{Arc, RwLock};

use pyo3::exceptions::PyTypeError;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyType};

use crate::errors::compression_error::CompressionError;
use crate::errors::parsing_error::ParsingError;
use crate::errors::version_error::VersionError;
use crate::retrievers::retriever::Retriever;
use crate::retrievers::retriever_combiner::RetrieverCombiner;
use crate::retrievers::retriever_ref::RetrieverRef;
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::r#struct::Struct;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs", subclass, eq)]
#[derive(Debug, Clone)]
pub struct BaseStruct {
    #[pyo3(get)]
    pub ver: Version,
    pub data: Arc<RwLock<Vec<Option<ParseableType>>>>,
    pub repeats: Arc<RwLock<Vec<Option<isize>>>>,
}

impl PartialEq for BaseStruct {
    fn eq(&self, other: &Self) -> bool {
        let data1 = self.data.read().expect("GIL bound read");
        let data2 = other.data.read().expect("GIL bound read");
        if data1.len() != data2.len() {
            return false
        }

        data1.iter().zip(data2.iter())
            .map(|(a, b)| a == b)
            .all(|x| x)
    }
}

impl Eq for BaseStruct {}

impl BaseStruct {
    pub fn new(ver: Version, data: Vec<Option<ParseableType>>, repeats: Vec<Option<isize>>) -> Self {
        BaseStruct {
            ver,
            data: Arc::new(RwLock::new(data)),
            repeats: Arc::new(RwLock::new(repeats))
        }
    }

    pub fn len(cls: &Bound<PyType>) -> PyResult<usize> {
        let struct_ = cls
            .getattr(intern!(cls.py(), "struct")).expect("always a BaseStruct subclass")
            .extract::<Struct>().expect("infallible");

        let retrievers = struct_.retrievers.read().expect("immutable");
        Ok(retrievers.len())
    }
    
    pub fn with_cls<'py>(val: BaseStruct, cls: &Bound<'py, PyType>) -> Bound<'py, PyAny> {
        let obj = cls.call((Version::new(vec![-1]), false), None).expect("always a BaseStruct subclass");
        *(obj.downcast::<BaseStruct>().expect("infallible").borrow_mut()) = val;
        obj
    }
    
    pub fn add_ret(cls: &Bound<PyType>, retriever: &Bound<Retriever>) -> PyResult<()> {
        if !cls.is_subclass_of::<BaseStruct>()? {
            return Err(PyTypeError::new_err(
                "Cannot create retrievers in classes that do not subclass BaseStruct"
            ))
        }
        let struct_ = match cls.getattr(intern!(cls.py(), "struct")) {
            Ok(struct_) => struct_.downcast_into::<Struct>()?,
            Err(_) => {
                let struct_ = Bound::new(cls.py(), Struct::new(cls.extract()?, cls.fully_qualified_name()?.to_string()))?;
                cls.setattr("struct", &struct_)?;
                struct_
            },
        }.borrow();
        let idx = struct_.add_ret(retriever)?;
        retriever.borrow_mut().idx = idx;
        Ok(())
    }
    
    pub fn add_comb(cls: &Bound<PyType>, retriever: &Bound<RetrieverCombiner>) -> PyResult<()> {
        let struct_ = match cls.getattr(intern!(cls.py(), "struct")) {
            Ok(struct_) => struct_.downcast_into::<Struct>()?,
            Err(_) => {
                return Err(PyTypeError::new_err(
                    "Cannot create combiners in classes that do not subclass BaseStruct. Note that the first retriever in a BaseStruct cannot be a ref or a combiner"
                ))
            },
        }.borrow();
        struct_.add_comb(retriever)
    }
    
    pub fn add_ref(cls: &Bound<PyType>, retriever: &Bound<RetrieverRef>) -> PyResult<()> {
        let struct_ = match cls.getattr(intern!(cls.py(), "struct")) {
            Ok(struct_) => struct_.downcast_into::<Struct>()?,
            Err(_) => {
                return Err(PyTypeError::new_err(
                    "Cannot create refs in classes that do not subclass BaseStruct or Manager. Note that the first retriever in a BaseStruct cannot be a ref or a combiner"
                ))
            },
        }.borrow();
        struct_.add_ref(retriever)
    }

    fn to_bytes<'py>(cls: &Bound<'py, PyType>, value: &BaseStruct) -> PyResult<Vec<u8>> {
        let struct_ = Struct::from_cls(cls)?;
        Ok(struct_.to_bytes(value)?)
    }
}

#[pymethods]
impl BaseStruct {
    #[new]
    #[classmethod]
    #[pyo3(signature = (ver = Version::new(vec![-1]), init_defaults = true, **retriever_inits))]
    fn new_py(cls: &Bound<PyType>, ver: Version, init_defaults: bool, retriever_inits: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let len = BaseStruct::len(cls)?;
        let mut data = vec![None; len];
        let repeats = vec![None; len];

        if !init_defaults {
            return Ok(BaseStruct::new(ver, data, repeats));
        }

        let struct_ = Struct::from_cls(cls)?;
        let retrievers = struct_.retrievers.read().expect("GIL bound read");

        for ret in retrievers.iter() {
            if !ret.supported(&ver) {
                continue;
            }
            let mut init = retriever_inits
                .and_then(|di| di.get_item(&ret.name).unwrap_or(None))
                .map(|obj| ret.data_type.to_parseable(&obj))
                .transpose()?;

            if init.is_none() {
                init = Some(ret.from_default(&ver, &repeats, cls.py())?);
            }

            data[ret.idx] = init;
        }
        Ok(BaseStruct::new(ver, data, repeats))
    }

    #[classmethod]
    #[pyo3(signature = (stream, ver = Version::new(vec![0,])))]
    fn from_stream<'py>(cls: &Bound<'py, PyType>, stream: &mut ByteStream, ver: Version) -> PyResult<Bound<'py, PyAny>> {
        let struct_ = Struct::from_cls(cls)?;

        let base = struct_.from_stream(stream, &ver)?;
        Ok(BaseStruct::with_cls(base, cls))
    }

    #[classmethod]
    #[pyo3(name = "to_bytes")]
    fn to_bytes_py<'py>(cls: &Bound<'py, PyType>, value: &BaseStruct) -> PyResult<Bound<'py, PyAny>> {
        let struct_ = Struct::from_cls(cls)?;
        Ok(PyBytes::new_bound(cls.py(), &struct_.to_bytes(value)?).into_any())
    }

    #[classmethod]
    fn from_bytes<'py>(cls: &Bound<'py, PyType>, bytes: &[u8]) -> PyResult<Bound<'py, PyAny>> {
        let mut stream = ByteStream::from_bytes(bytes);
        BaseStruct::from_stream(cls, &mut stream, Version::new(vec![0, ]))
    }

    #[classmethod]
    #[pyo3(signature = (filepath, strict = true))]
    fn from_file<'py>(cls: &Bound<'py, PyType>, filepath: &str, strict: bool) -> PyResult<Bound<'py, PyAny>> {
        let mut stream = ByteStream::from_file(filepath)?;
        let struct_ = BaseStruct::from_stream(cls, &mut stream, Version::new(vec![0, ]))?;
        
        if !strict {
            return Ok(struct_);
        }
        
        let rem = stream.remaining().len();
        if rem > 0 {
            return Err(ParsingError::new_err(format!("{rem} bytes are left after parsing all retrievers successfully")))
        }
        
        Ok(struct_)
    }

    #[classmethod]
    fn to_file(cls: &Bound<PyType>, filepath: &str, value: &BaseStruct) -> PyResult<()> {
        let bytes = Self::to_bytes(cls, value)?;
        let mut file = File::create(filepath)?;
        Ok(file.write_all(&bytes)?)
    }

    #[classmethod]
    #[pyo3(signature = (_stream, _ver = Version::new(vec![0,])))]
    fn _get_version(_cls: &Bound<PyType>, _stream: &mut ByteStream, _ver: Version) -> PyResult<Version> {
        Err(VersionError::new_err("Un-versioned File"))
    }

    #[classmethod]
    fn _compress(_cls: &Bound<PyType>, _bytes: &[u8]) -> PyResult<Vec<u8>> {
        Err(CompressionError::new_err(
            "Unable to write object to file. A Structure with a compressed section needs to implement '_compress' classmethod."
        ))
    }

    #[classmethod]
    fn _decompress(_cls: &Bound<PyType>, _bytes: &[u8]) -> PyResult<Vec<u8>> {
        Err(CompressionError::new_err(
            "Unable to read object from file. A Structure with a compressed section needs to implement '_decompress' classmethod."
        ))
    }
}
