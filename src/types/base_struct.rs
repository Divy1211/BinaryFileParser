use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, BufWriter, Write};
use std::sync::{Arc, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use pyo3::exceptions::{PyAttributeError, PyTypeError, PyValueError};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList, PyTuple, PyType};
use serde::de::DeserializeSeed;
use serde_json::Deserializer;

use crate::errors::compression_error::CompressionError;
use crate::errors::default_attribute_error::DefaultAttributeError;
use crate::errors::parsing_error::ParsingError;
use crate::errors::version_error::VersionError;
use crate::retrievers::retriever::Retriever;
use crate::retrievers::retriever_combiner::RetrieverCombiner;
use crate::retrievers::retriever_ref::RetrieverRef;
use crate::types::byte_stream::ByteStream;
use crate::types::context::{Context, ContextPtr};
use crate::types::diff::diff::Diffable;
use crate::types::diff::merge::Mergeable;
use crate::types::diff::struct_diffable::StructDiffable;
use crate::types::diff::struct_mergeable::StructMergeable;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::serial::struct_deserializer::StructDeserializer;
use crate::types::serial::struct_serializer::StructSerializer;
use crate::types::struct_builder::StructBuilder;
use crate::types::version::Version;

#[derive(Debug)]
pub struct BaseStructRaw {
    pub ver: Version,
    pub data: Vec<Option<ParseableType>>,
    pub repeats: Vec<Option<isize>>,
    pub obj: OnceLock<Py<PyAny>>
}

impl BaseStructRaw {
    // todo: refactor usages of this function to just take a reference to raw. This is a big chore
    pub fn split(&mut self) -> (&mut Vec<Option<ParseableType>>, &mut Vec<Option<isize>>, &Version) {
        (&mut self.data, &mut self.repeats, &self.ver)
    }
}

#[pyclass(module = "bfp_rs", subclass, eq)]
#[derive(Debug, Clone)]
pub struct BaseStruct {
    raw: Arc<RwLock<BaseStructRaw>>,
}

impl PartialEq for BaseStruct {
    fn eq(&self, other: &Self) -> bool {
        let data1 = &self.inner().data;
        let data2 = &other.inner().data;

        if data1.len() != data2.len() {
            return false
        }
        
        data1.iter().zip(data2.iter())
            .all(|(a, b)| a == b)
    }
}

impl Eq for BaseStruct {}

impl BaseStruct {
    pub fn inner(&self) -> RwLockReadGuard<BaseStructRaw> {
        self.raw.read().expect("GIL bound read")
    }

    pub fn inner_mut(&self) -> RwLockWriteGuard<BaseStructRaw> {
        self.raw.write().expect("GIL bound write")
    }
    
    pub fn new(ver: Version, data: Vec<Option<ParseableType>>, repeats: Vec<Option<isize>>) -> Self {
        BaseStruct { raw: Arc::new(RwLock::new(BaseStructRaw {
            ver,
            data,
            repeats,
            obj: OnceLock::new()
        }))}
    }

    pub fn len(cls: &Bound<PyType>) -> PyResult<usize> {
        let struct_ = StructBuilder::get_struct(cls)?;

        let retrievers = struct_.retrievers();
        Ok(retrievers.len())
    }

    // todo: figure out unsafe allocations
    pub fn with_cls<'py>(val: BaseStruct, cls: &Bound<'py, PyType>) -> PyResult<Bound<'py, PyAny>> {
        let kwargs = PyDict::new(cls.py());
        kwargs.set_item("ver", Version::new(vec![-1]).into_pyobject(cls.py())?)?;
        kwargs.set_item("ctx", ContextPtr::new().into_pyobject(cls.py())?)?;
        kwargs.set_item("init_defaults", false)?;
        let obj = cls.call_method(intern!(cls.py(), "__new__"), (cls,), Some(&kwargs))?;
        let name = intern!(cls.py(), "__reconstruct__");
        if cls.hasattr(name)? {
            cls.call_method(name, (&obj,), None)?;
        }
        *(obj.cast::<BaseStruct>().expect("always a BaseStruct subclass").borrow_mut()) = val;
        Ok(obj)
    }
    
    pub fn add_ret(cls: &Bound<PyType>, retriever: &Bound<Retriever>) -> PyResult<()> {
        if !cls.is_subclass_of::<BaseStruct>()? {
            return Err(PyTypeError::new_err(
                "Cannot create retrievers in classes that do not subclass BaseStruct"
            ))
        }
        let mut struct_ = match cls.getattr(intern!(cls.py(), "__struct_builder__")) {
            Ok(struct_) => struct_.cast_into::<StructBuilder>()?,
            Err(_) => {
                let struct_ = Bound::new(cls.py(), StructBuilder::new())?;
                cls.setattr("__struct_builder__", &struct_)?;
                struct_
            },
        }.borrow_mut();
        let idx = struct_.add_ret(retriever)?;
        retriever.borrow_mut().idx = idx;
        Ok(())
    }
    
    pub fn add_comb(cls: &Bound<PyType>, retriever: &Bound<RetrieverCombiner>) -> PyResult<()> {
        let mut struct_ = match cls.getattr(intern!(cls.py(), "__struct_builder__")) {
            Ok(struct_) => struct_.cast_into::<StructBuilder>()?,
            Err(_) => {
                return Err(PyTypeError::new_err(
                    "Cannot create combiners in classes that do not subclass BaseStruct. Note that the first retriever in a BaseStruct cannot be a ref or a combiner"
                ))
            },
        }.borrow_mut();
        struct_.add_comb(retriever)
    }
    
    pub fn add_ref(cls: &Bound<PyType>, retriever: &Bound<RetrieverRef>) -> PyResult<()> {
        let mut struct_ = match cls.getattr(intern!(cls.py(), "__struct_builder__")) {
            Ok(struct_) => struct_.cast_into::<StructBuilder>()?,
            Err(_) => {
                return Err(PyTypeError::new_err(
                    "Cannot create refs in classes that do not subclass BaseStruct or RefStruct. Note that the first retriever in a BaseStruct cannot be a ref or a combiner"
                ))
            },
        }.borrow_mut();
        struct_.add_ref(retriever)
    }

    fn to_bytes(cls: &Bound<'_, PyType>, value: &BaseStruct, filepath: &str) -> PyResult<Vec<u8>> {
        let struct_ = StructBuilder::get_struct(cls)?;
        let bar = MultiProgress::new();
        
        let spinner = bar.add(ProgressBar::new_spinner());
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner} {msg}")
                .unwrap(),
        );
        spinner.set_message(format!("⬅ Writing File '{}'", filepath));
        spinner.enable_steady_tick(Duration::from_millis(100));
        
        let mut bytes_ = Vec::new();
        
        struct_.to_bytes_(value, Some(bar), &mut bytes_)?;

        if struct_.is_compressed() {
            struct_.compress(&mut bytes_, 0)?
        }
        
        spinner.set_message(format!("✔ Finished Writing File '{}'", filepath));
        spinner.finish();
        
        Ok(bytes_)
    }

    fn from_stream_<'py>(cls: &Bound<'py, PyType>, stream: &mut ByteStream, ver: Version, filepath: Option<&str>) -> PyResult<Bound<'py, PyAny>> {
        let struct_ = StructBuilder::get_struct(cls)?;

        if struct_.is_compressed() {
            *stream = struct_.decompress(stream.remaining())?;
        }
        
        let Some(filepath) = filepath else {
            let base = struct_.from_stream_(stream, &ver, None, &mut Context::new())?;
            return BaseStruct::with_cls(base, cls);
        };
        
        
        let bar =  MultiProgress::new();
        let spinner = bar.add(ProgressBar::new_spinner());
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner} {msg}")
                .unwrap(),
        );
        spinner.set_message(format!("➡ Reading File '{}'", filepath));
        spinner.enable_steady_tick(Duration::from_millis(100));
        
        let base = struct_.from_stream_(stream, &ver, Some(bar), &mut Context::new())?;
        
        spinner.set_message(format!("✔ Finished Reading File '{}'", filepath));
        spinner.finish();
        
        BaseStruct::with_cls(base, cls)
    }
}

#[pymethods]
impl BaseStruct {
    #[getter]
    #[pyo3(name = "ver")]
    fn ver_py(slf: PyRef<Self>) -> Version {
        slf.raw.read().expect("GIL bound read").ver.clone()
    }
    
    #[classmethod]
    fn retrievers<'py>(cls: &Bound<'py, PyType>) -> PyResult<Bound<'py, PyList>> {
        let struct_ = StructBuilder::get_struct(cls)?;
        PyList::new(
            cls.py(),
            struct_.retrievers()
                .iter()
                .map(|x| x.clone().into_pyobject(cls.py()))
                .collect::<Result<Vec<_>, _>>()?
        )
    }

    #[new]
    #[classmethod]
    #[pyo3(signature = (*_args, ver = Version::new(vec![-1]), ctx = ContextPtr::new(), init_defaults = true, **retriever_inits))]
    fn new_py(
        cls: &Bound<PyType>,
        _args: &Bound<'_, PyTuple>,
        mut ver: Version, ctx: ContextPtr,
        init_defaults: bool,
        retriever_inits: Option<&Bound<'_, PyDict>>
    ) -> PyResult<Self> {
        if ver == Version::new(vec![-1]) {
            ver = match cls.getattr("__default_ver__") {
                Ok(val) => {
                    val.extract().map_err(|_| PyTypeError::new_err(format!(
                        "Error while reading __default_ver__: '{}' cannot be converted to 'Version'",
                        val.get_type().name().map_or("<unknown>".into(), |val| { val.to_string() })
                    )))
                },
                Err(err) if err.is_instance_of::<PyAttributeError>(cls.py()) => Ok(ver),
                Err(err) => Err(err),
            }?;
        }

        let len = BaseStruct::len(cls)?;
        let mut data = vec![None; len];
        let mut repeats = vec![None; len];

        if !init_defaults {
            return Ok(BaseStruct::new(ver, data, repeats));
        }

        let struct_ = StructBuilder::get_struct(cls)?;
        let retrievers = &struct_.raw.retrievers;
        
        for ret in retrievers.iter() {
            if !ret.supported(&ver) {
                continue;
            }
            let mut init = retriever_inits
                .and_then(|di| di.get_item(&ret.name).unwrap_or(None))
                .map(|obj| ret.data_type.to_parseable(&obj))
                .transpose()?;
            
            if init.is_none() {
                init = match ret.from_default(&ver, &mut repeats, &ctx, cls.py()) {
                    Ok(val) => Some(val),
                    Err(e) => {
                        let err = DefaultAttributeError::new_err(format!(
                            "Error occurred during initialization of default value for property '{}'", ret.name
                        ));
                        err.set_cause(cls.py(), Some(e));
                        return Err(err);
                    }
                };
            }

            data[ret.idx] = init;
            
            let mut ctx = ctx.inner.write().expect("GIL bound write");
            ret.call_on_reads(retrievers, &mut data, &mut repeats, &ver, &mut ctx)?;
        }
        Ok(BaseStruct::new(ver, data, repeats))
    }

    #[classmethod]
    pub fn from_base<'py>(cls: &Bound<'py, PyType>, val: BaseStruct) -> PyResult<Bound<'py, PyAny>> {
        Self::with_cls(val, cls)
    }
    
    #[classmethod]
    #[pyo3(signature = (stream, ver = Version::new(vec![0,])))]
    fn from_stream<'py>(cls: &Bound<'py, PyType>, stream: &mut ByteStream, ver: Version) -> PyResult<Bound<'py, PyAny>> {
        BaseStruct::from_stream_(cls, stream, ver, None)
    }
    
    #[pyo3(name = "to_bytes")]
    fn to_bytes_py(slf: Bound<'_, Self>) -> PyResult<Bound<'_, PyAny>> {
        let value = slf.extract()?;
        let slf = slf.into_any();
        let cls = slf.get_type();
        
        let struct_ = StructBuilder::get_struct(&cls)?;
        let mut bytes = struct_.to_bytes(&value)?;
        if struct_.is_compressed() {
            struct_.compress(&mut bytes, 0)?;
        }
        Ok(PyBytes::new(cls.py(), &bytes).into_any())
    }

    #[classmethod]
    #[pyo3(signature = (bytes, ver = Version::new(vec![0, ]), strict = false))]
    fn from_bytes<'py>(cls: &Bound<'py, PyType>, bytes: &[u8], ver: Version, strict: bool) -> PyResult<Bound<'py, PyAny>> {
        let mut stream = ByteStream::from_bytes(bytes);
        let struct_ = BaseStruct::from_stream(cls, &mut stream, ver)?;

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
    #[pyo3(signature = (filepath, ver = Version::new(vec![0, ]), strict = true))]
    fn from_file<'py>(cls: &Bound<'py, PyType>, filepath: &str, ver: Version, strict: bool) -> PyResult<Bound<'py, PyAny>> {
        let mut stream = ByteStream::from_file(filepath)?;
        let struct_ = BaseStruct::from_stream_(cls, &mut stream, ver, Some(filepath))?;
        
        if !strict {
            return Ok(struct_);
        }
        
        let rem = stream.remaining().len();
        if rem > 0 {
            return Err(ParsingError::new_err(format!("{rem} bytes are left after parsing all retrievers successfully")))
        }
        
        Ok(struct_)
    }

    fn to_file(slf: Bound<Self>, filepath: &str) -> PyResult<()> {
        let value = slf.extract()?;
        let slf = slf.into_any();
        let cls = slf.get_type();

        let bytes = Self::to_bytes(&cls, &value, filepath)?;
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

    fn to_json(slf: Bound<Self>, filepath: &str) -> PyResult<()> {
        let value = slf.extract()?;
        let slf = slf.into_any();
        let cls = slf.get_type();

        let struct_ = StructBuilder::get_struct(&cls)?;
        let serializer = StructSerializer(&struct_, &value);

        let file = File::create(filepath)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, &serializer)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    #[classmethod]
    fn from_json<'py>(cls: &Bound<'py, PyType>, filepath: &str) -> PyResult<Bound<'py, PyAny>> {
        let struct_ = StructBuilder::get_struct(cls)?;
        let mut ctx = Context::new();
        let deserializer = StructDeserializer(&struct_, &mut ctx);

        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        let mut de = Deserializer::from_reader(reader);

        let val = deserializer.deserialize(&mut de).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        BaseStruct::with_cls(val, cls)
    }

    fn diff<'py>(slf: Bound<'py, Self>, other: Bound<Self>) -> PyResult<Bound<'py, PyDict>> {
        let value1 = slf.extract()?;
        let value2 = other.extract()?;

        let slf = slf.into_any();
        let cls1 = slf.get_type();
        let other = other.into_any();
        let cls2 = other.get_type();

        let struct1 = StructBuilder::get_struct(&cls1)?;
        let struct2 = StructBuilder::get_struct(&cls2)?;

        if struct1 != struct2 {
            return Err(
                PyTypeError::new_err(format!(
                    "Cannot diff structs of different classes '{}' and '{}'",
                    struct1.raw.fully_qualified_name,
                    struct2.raw.fully_qualified_name,
                )))
        }

        let diff1 = StructDiffable(&struct1, &value1);
        let diff2 = StructDiffable(&struct2, &value2);
        
        let result = diff1.diff(&diff2);
        
        diff1.to_dict(result, slf.py())
    }

    fn merge<'py>(base: Bound<'py, Self>, slf: Bound<Self>, other: Bound<Self>) -> PyResult<Bound<'py, PyDict>> {
        let mut value0 = base.extract::<BaseStruct>()?;
        let mut value1 = slf.extract::<BaseStruct>()?;
        let mut value2 = other.extract::<BaseStruct>()?;

        let base = base.into_any();
        let cls0 = base.get_type();
        let slf = slf.into_any();
        let cls1 = slf.get_type();
        let other = other.into_any();
        let cls2 = other.get_type();

        let struct0 = StructBuilder::get_struct(&cls0)?;
        let struct1 = StructBuilder::get_struct(&cls1)?;
        let struct2 = StructBuilder::get_struct(&cls2)?;

        if struct0 != struct1 || struct1 != struct2 {
            return Err(
                PyTypeError::new_err(format!(
                    "Cannot merge structs of different classes '{}', '{}', and {}",
                    struct0.raw.fully_qualified_name,
                    struct1.raw.fully_qualified_name,
                    struct2.raw.fully_qualified_name,
                )))
        }

        if value0.inner().ver != value1.inner().ver || value1.inner().ver != value2.inner().ver {
            return Err(
                PyValueError::new_err(format!(
                    "Cannot merge structs of different versions '{}', '{}', and {}",
                    value0.inner().ver,
                    value1.inner().ver,
                    value2.inner().ver,
                )))
        }
        
        let mut merge0 = StructMergeable(&struct0, &mut value0);
        let merge1 = StructMergeable(&struct1, &mut value1);
        let merge2 = StructMergeable(&struct2, &mut value2);
        
        let conflicts = merge0.merge(&merge1, &merge2);
        
        merge0.to_dict(conflicts, base.py())
    }
}

impl Hash for BaseStruct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().data.hash(state);
    }
}