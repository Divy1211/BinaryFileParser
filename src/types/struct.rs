use std::sync::{Arc, RwLock};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString, PyType};

use crate::errors::compression_error::CompressionError;
use crate::errors::version_error::VersionError;
use crate::retrievers::retriever::{RetState, Retriever};
use crate::retrievers::retriever_combiner::RetrieverCombiner;
use crate::retrievers::retriever_ref::RetrieverRef;
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_list::BfpList;
use crate::types::byte_stream::ByteStream;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

// todo: make an inner
#[pyclass(module = "bfp_rs", eq)]
#[derive(Debug, Clone)]
pub struct Struct {
    pub retrievers: Arc<RwLock<Vec<Retriever>>>,
    pub combiners: Arc<RwLock<Vec<RetrieverCombiner>>>,
    pub refs: Arc<RwLock<Vec<RetrieverRef>>>,
    
    pub py_type: Arc<Py<PyType>>,
    pub fully_qualified_name: String,
    
    pub get_ver: Option<Arc<PyObject>>,
    pub compress: Option<Arc<PyObject>>,
    pub decompress: Option<Arc<PyObject>>,
}

impl PartialEq for Struct {
    fn eq(&self, other: &Self) -> bool {
        self.fully_qualified_name == other.fully_qualified_name
    }
}

impl Eq for Struct {}

impl Struct {
    pub fn new(py_type: Py<PyType>, fully_qualified_name: String) -> Self {
        Struct {
            retrievers: Arc::new(RwLock::new(Vec::new())),
            combiners: Arc::new(RwLock::new(Vec::new())),
            refs: Arc::new(RwLock::new(Vec::new())),
            
            py_type: Arc::new(py_type),
            fully_qualified_name,
            
            get_ver: None,
            compress: None,
            decompress: None,
        }
    }

    pub fn add_ret(&self, retriever: &Bound<Retriever>) -> PyResult<usize> {
        let mut retriever = retriever.extract::<Retriever>()?;
        let mut retrievers = self.retrievers.write().expect("GIL bound write");
        let idx = retrievers.len();
        retriever.idx = idx;
        retrievers.push(retriever);
        Ok(idx)
    }

    pub fn add_comb(&self, combiner: &Bound<RetrieverCombiner>) -> PyResult<()> {
        let combiner = combiner.extract::<RetrieverCombiner>()?;
        let mut combiners = self.combiners.write().expect("GIL bound write");
        combiners.push(combiner);
        Ok(())
    }

    pub fn add_ref(&self, ref_: &Bound<RetrieverRef>) -> PyResult<()> {
        let ref_ = ref_.extract::<RetrieverRef>()?;
        let mut refs = self.refs.write().expect("GIL bound write");
        refs.push(ref_);
        Ok(())
    }

    pub fn from_cls(cls: &Bound<PyType>) -> PyResult<Self> {
        let mut struct_ = cls
            .getattr(intern!(cls.py(), "struct")).expect("always a BaseStruct subclass")
            .extract::<Struct>().expect("infallible");

        struct_.get_ver = get_if_impl(cls, intern!(cls.py(), "_get_version"));
        struct_.compress = get_if_impl(cls, intern!(cls.py(), "_compress"));
        struct_.decompress = get_if_impl(cls, intern!(cls.py(), "_decompress"));

        for retriever in struct_.retrievers.write().expect("GIL bound write").iter_mut() {
            retriever.construct_fns(cls.py())?
        }
        
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

    pub fn decompress(&self, bytes: &[u8]) -> PyResult<ByteStream> {
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

    pub fn compress(&self, bytes: &mut Vec<u8>, idx: usize) -> PyResult<()> {
        let Some(fn_) = &self.compress else {
            return Err(CompressionError::new_err(
                "Unable to write object to file. A Structure with a compressed section needs to implement '_compress' classmethod."
            ))
        };

        Python::with_gil(|py| {
            let py_bytes = fn_.call_bound(py, (PyBytes::new_bound(py, &bytes[idx..]),), None)?;
            bytes.truncate(idx);
            bytes.extend_from_slice(py_bytes.extract::<&[u8]>(py)?);
            Ok(())
        })
    }

    pub fn from_stream_(&self, stream: &mut ByteStream, ver: &Version, bar: Option<MultiProgress>) -> std::io::Result<BaseStruct> {
        let retrievers = self.retrievers.read().expect("immutable"); // todo: change to Arc<Vec<>> with builder pattern?
        let mut data = Vec::with_capacity(retrievers.len());
        let mut repeats = vec![None; retrievers.len()];

        let ver = self.get_ver(stream, ver)?;

        let mut progress = None;
        if let Some(bar) = bar {
            let pb = bar.add(ProgressBar::new(retrievers.len() as u64));
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("  [{bar:40.cyan/blue}] {pos}/{len}{msg}")
                    .unwrap(),
            );
            progress = Some(pb);
        }
        
        for (i, retriever) in retrievers.iter().enumerate() {
            if retriever.remaining_compressed {
                *stream = self.decompress(stream.remaining())?
            }
            if let Some(progress) = progress.as_ref() {
                progress.set_message(format!("\n    ➡ Reading '{}'", retriever.name));
                progress.set_position((i+1) as u64);
            }
            if !retriever.supported(&ver) {
                data.push(None);
                continue;
            }

            data.push(Some(match retriever.state(&repeats) {
                RetState::NoneValue | RetState::NoneList => { ParseableType::None }
                RetState::Value => { retriever.from_stream(stream, &ver)? }
                RetState::List => {
                    let mut ls = Vec::with_capacity(retriever.repeat(&repeats) as usize);
                    for _ in 0..retriever.repeat(&repeats) {
                        ls.push(retriever.from_stream(stream, &ver)?);
                    }
                    BfpList::new(ls, retriever.data_type.clone()).into()
                }
            }));

            retriever.call_on_reads(&retrievers, &mut data, &mut repeats, &ver)?;

            if let Some(progress) = progress.as_ref() {
                progress.set_message("");
                progress.finish();
            }
        }
        Ok(BaseStruct::new(ver.clone(), data, repeats))
    }

    pub fn to_bytes_(&self, value: &BaseStruct, bar: Option<MultiProgress>) -> std::io::Result<Vec<u8>> {
        let mut data_lock = value.data.write().expect("GIL bound write");
        let mut repeats_lock = value.repeats.write().expect("GIL bound write");

        let data = data_lock.as_mut();
        let repeats = repeats_lock.as_mut();
        let retrievers = self.retrievers.read().expect("immutable");

        let mut bytes = Vec::with_capacity(retrievers.len());
        let mut compress_idx = None;

        let mut progress = None;
        if let Some(bar) = bar {
            let pb = bar.add(ProgressBar::new(retrievers.len() as u64));
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("  [{bar:40.cyan/blue}] {pos}/{len}{msg}")
                    .unwrap(),
            );
            progress = Some(pb);
        }
        
        for (i, retriever) in retrievers.iter().enumerate() {
            if !retriever.supported(&value.ver) {
                continue;
            }
            if let Some(progress) = progress.as_ref() {
                progress.set_message(format!("\n    ⬅ Writing '{}'", retriever.name));
                progress.set_position((i+1) as u64);
            }

            if retriever.remaining_compressed {
                compress_idx = Some(bytes.len());
            }
            retriever.call_on_writes(&retrievers, data, repeats, &value.ver)?;

            let value = data[retriever.idx].as_ref().expect("supported check done above");

            bytes.append(&mut match retriever.state(repeats) {
                RetState::NoneList | RetState::NoneValue => { vec![] }
                RetState::Value => {
                    retriever.to_bytes(value)?
                }
                RetState::List => {
                    let ParseableType::Array(ls) = value else {
                        unreachable!("Retriever state guarantee broken while reading '{}'", retriever.name)
                    };
                    let ls = ls.ls.read().expect("GIL bound read");
                    let mut bytes = Vec::with_capacity(ls.len());
                    for item in ls.iter() {
                        bytes.append(&mut retriever.to_bytes(item)?);
                    }
                    bytes
                }
            });
        }
        
        if let Some(progress) = progress.as_ref() {
            progress.set_message("");
            progress.finish();
        }
        
        if let Some(idx) = compress_idx {
            self.compress(&mut bytes, idx)?;
        }
        Ok(bytes)
    }
}

impl Parseable for Struct {
    type Type = BaseStruct;
    
    fn from_stream(&self, stream: &mut ByteStream, ver: &Version) -> std::io::Result<BaseStruct> {
        self.from_stream_(stream, ver, None)
    }

    fn to_bytes(&self, value: &BaseStruct) -> std::io::Result<Vec<u8>> {
        self.to_bytes_(value, None)
    }
}


fn get_if_impl(cls: &Bound<PyType>, attr: &Bound<PyString>) -> Option<Arc<PyObject>> {
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
