use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{DeserializeSeed, MapAccess, Visitor};
use serde::ser::SerializeMap;
use crate::errors::compression_error::CompressionError;
use crate::errors::parsing_error::ParsingError;
use crate::retrievers::retriever::{RetState, Retriever};
use crate::retrievers::retriever_combiner::RetrieverCombiner;
use crate::retrievers::retriever_ref::RetrieverRef;
use crate::types::base_struct::BaseStruct;
use crate::types::bfp_list::BfpList;
use crate::types::bfp_type::{ArraySeed, TypeDeserializer};
use crate::types::byte_stream::ByteStream;
use crate::types::context::Context;
use crate::types::parseable::Parseable;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[derive(Debug)]
pub struct StructRaw {
    pub retrievers: Vec<Retriever>,
    #[allow(unused)]
    pub combiners: Vec<RetrieverCombiner>,
    #[allow(unused)]
    pub refs: Vec<RetrieverRef>,
    
    pub py_type: Py<PyType>,
    pub fully_qualified_name: String,
    
    pub is_compressed: bool,
    
    pub get_ver: Option<PyObject>,
    pub compress: Option<PyObject>,
    pub decompress: Option<PyObject>,
}

#[pyclass(module = "bfp_rs", eq)]
#[derive(Debug, Clone)]
pub struct Struct {
    raw: Arc<StructRaw>
}

impl PartialEq for Struct {
    fn eq(&self, other: &Self) -> bool {
        self.raw.fully_qualified_name == other.raw.fully_qualified_name
    }
}

impl Eq for Struct {}

impl Struct {
    pub fn is_compressed(&self) -> bool {
        self.raw.is_compressed
    }
    
    pub fn from_raw(raw: StructRaw) -> Self {
        Self { raw: Arc::new(raw) }
    }
    
    pub fn retrievers(&self) -> &[Retriever] {
        &self.raw.retrievers
    }
    
    pub fn fully_qualified_name(&self) -> String {
        self.raw.fully_qualified_name.clone()
    }

    pub fn py_type<'py>(&self, py: Python<'py>) -> &Bound<'py, PyType> {
        self.raw.py_type.bind(py)
    }
    
    pub fn get_ver<'a>(&self, stream: &mut ByteStream, ver: &'a Version) -> PyResult<Version> {
        let Some(fn_) = &self.raw.get_ver else {
            return Ok(ver.clone())
        };
        
        Python::with_gil(|py| {
            let ver = fn_.call_bound(py, (stream.clone(), ver.clone()), None)?;
            ver.extract::<Version>(py)
        })
    }

    pub fn decompress(&self, bytes: &[u8]) -> PyResult<ByteStream> {
        let Some(fn_) = &self.raw.decompress else {
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
        let Some(fn_) = &self.raw.compress else {
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

    pub fn from_stream_(
        &self,
        stream: &mut ByteStream,
        ver: &Version,
        bar: Option<MultiProgress>,
        ctx: &mut Context
    ) -> PyResult<BaseStruct> {
        let retrievers = &self.raw.retrievers;
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
            if !retriever.supported(&ver) {
                data.push(None);
                continue;
            }

            if let Some(progress) = progress.as_ref() {
                progress.set_message(format!("\n    ➡ Reading '{}'", retriever.name));
                progress.set_position((i+1) as u64);
            }

            data.push(Some(match retriever.state(&repeats) {
                RetState::NoneValue | RetState::NoneList => { ParseableType::None }
                RetState::Value => {
                    retriever.from_stream_ctx(stream, &ver, ctx)
                        .map_err(|e| { Python::with_gil(|py| {
                            let err = ParsingError::new_err(format!("Error occurred while reading '{}'", retriever.name));
                            err.set_cause(py, Some(e));
                            err
                        }) })?
                }
                RetState::List => {
                    let mut ls = Vec::with_capacity(retriever.repeat(&repeats) as usize);
                    for i in 0..retriever.repeat(&repeats) {
                        ctx.idxes.push(i as usize);
                        ls.push(
                            retriever.from_stream_ctx(stream, &ver, ctx)
                                .map_err(|e| { Python::with_gil(|py| {
                                    let err = ParsingError::new_err(format!("Error occurred while reading '{}'", retriever.name));
                                    err.set_cause(py, Some(e));
                                    err
                                }) })?
                        );
                        ctx.idxes.pop();
                    }
                    BfpList::new(ls, retriever.data_type.clone()).into()
                }
            }));

            retriever.call_on_reads(&retrievers, &mut data, &mut repeats, &ver, ctx)?;

            if let Some(progress) = progress.as_ref() {
                progress.set_message("");
                progress.finish();
            }
        }
        Ok(BaseStruct::new(ver.clone(), data, repeats))
    }

    pub fn to_bytes_(&self, value: &BaseStruct, bar: Option<MultiProgress>, buffer: &mut Vec<u8>) -> PyResult<()> {
        let mut inner = value.inner_mut();

        let retrievers = &self.raw.retrievers;

        buffer.reserve(retrievers.len());
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
            if !retriever.supported(&inner.ver) {
                continue;
            }
            if let Some(progress) = progress.as_ref() {
                progress.set_message(format!("\n    ⬅ Writing '{}'", retriever.name));
                progress.set_position((i+1) as u64);
            }

            if retriever.remaining_compressed {
                compress_idx = Some(buffer.len());
            }

            let (data, repeats, ver) = inner.split();
            
            retriever.call_on_writes(&retrievers, data, repeats, ver)?;

            let value = inner.data[retriever.idx].as_ref().expect("supported check done above");

            match retriever.state(&inner.repeats) {
                RetState::NoneList | RetState::NoneValue => {},
                RetState::Value => {
                    retriever.to_bytes_in(value, buffer)?;
                }
                RetState::List => {
                    let ParseableType::Array(ls) = value else {
                        unreachable!("Retriever state guarantee broken while writing '{}'", retriever.name)
                    };
                    let inner = ls.inner();
                    for item in inner.data.iter() {
                        retriever.to_bytes_in(item, buffer)?;
                    }
                }
            }
        }
        
        if let Some(progress) = progress.as_ref() {
            progress.set_message("");
            progress.finish();
        }
        
        if let Some(idx) = compress_idx {
            self.compress(buffer, idx)?;
        }
        Ok(())
    }
}

impl Parseable for Struct {
    type Type = BaseStruct;

    fn from_stream_ctx(&self, stream: &mut ByteStream, ver: &Version, ctx: &mut Context) -> PyResult<Self::Type> {
        self.from_stream_(stream, ver, None, ctx)
    }

    fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()> {
        self.to_bytes_(value, None, buffer)
    }
}

pub struct SerdeSerializer<'a, 'b>(pub &'a Struct, pub &'b BaseStruct);
impl Serialize for SerdeSerializer<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let struct_ = self.0;
        let value = self.1;
        let mut inner = value.inner_mut();

        let retrievers = &struct_.raw.retrievers;
        
        let mut s = serializer.serialize_map(None)?;

        s.serialize_entry("ver", &inner.ver)?;
        for retriever in retrievers.iter() {
            if !retriever.supported(&inner.ver) {
                continue;
            }

            let (data, repeats, ver) = inner.split();

            retriever.call_on_writes(&retrievers, data, repeats, ver).map_err(|py_err| {
                serde::ser::Error::custom(format!(
                    "Python error during serialization: {}",
                    py_err
                ))
            })?;

            let value = inner.data[retriever.idx].as_ref().expect("supported check done above");
            
            match retriever.state(&inner.repeats) {
                RetState::NoneList | RetState::NoneValue => { s.serialize_entry(&retriever.name, &Option::<i32>::None)?; },
                RetState::Value => {
                    s.serialize_entry(&retriever.name, value)?;
                }
                RetState::List => {
                    let ParseableType::Array(ls) = value else {
                        unreachable!("Retriever state guarantee broken while writing '{}'", retriever.name)
                    };
                    s.serialize_entry(&retriever.name, ls)?;
                }
            }
        }
        s.end()
    }
}

pub struct SerdeDeserializer<'a, 'b>(pub &'a Struct, pub &'b mut Context);
impl<'de, 'a, 'b> DeserializeSeed<'de> for SerdeDeserializer<'a, 'b> {
    type Value = BaseStruct;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de, 'a, 'b> Visitor<'de> for SerdeDeserializer<'a, 'b> {
    type Value = BaseStruct;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a map matching retriever names")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let retrievers = &self.0.raw.retrievers;
        let ctx = self.1;
        let mut data = Vec::with_capacity(retrievers.len());
        let mut repeats = vec![None; retrievers.len()];

        let mut values = HashMap::with_capacity(retrievers.len() + 1);
        while let Some((key, value)) = access.next_entry::<String, serde_json::Value>()? {
            values.insert(key, value);
        }
        
        let Some(ver) = values.get("ver") else {
            return Err(serde::de::Error::custom("Invalid Object: Version not found"));
        };
        let ver = Version::deserialize(ver).map_err(|e| serde::de::Error::custom(e))?;
        
        for (i, retriever) in retrievers.iter().enumerate() {
            if !retriever.supported(&ver) {
                data.push(None);
                continue;
            }
            
            let Some(value) = values.remove(&retriever.name) else {
                return Err(serde::de::Error::custom(format!("Invalid Object: '{}' not found", retriever.name)));
            };

            data.push(Some(match retriever.state(&repeats) {
                RetState::Value | RetState::NoneValue if value.is_null() => {
                    repeats[i] = Some(-1);
                    ParseableType::None
                },
                RetState::List | RetState::NoneList if value.is_null() => {
                    repeats[i] = Some(-2);
                    ParseableType::None
                },
                RetState::Value | RetState::NoneValue => {
                    repeats[i] = None;
                    TypeDeserializer(&retriever.data_type, ctx)
                        .deserialize(value)
                        .map_err(|e| serde::de::Error::custom(format!("Error occurred while reading '{}': {e}", retriever.name)))?
                }
                RetState::List | RetState::NoneList => {
                    let repeat = retriever.repeat(&repeats);
                    if !value.is_array() {
                        return Err(serde::de::Error::custom(format!(
                            "Invalid Object: {} should be an array", retriever.name
                        )))
                    }
                    let len = value.as_array().expect("Infallible").len() as isize;
                    if repeat == -2 {
                        repeats[i] = Some(len);
                    } else if repeats[i].is_none() && repeat != len {
                        return Err(serde::de::Error::custom(format!(
                            "List length mismatch for '{}' which is a retriever of fixed repeat. Expected: {repeat}, Actual: {len}", retriever.name
                        )))
                    }
                    let ls = ArraySeed(TypeDeserializer(&retriever.data_type, ctx))
                        .deserialize(value)
                        .map_err(|e| serde::de::Error::custom(format!("Error occurred while reading '{}': {e}", retriever.name)))?;
                    ParseableType::Array(ls)
                }
            }));
            retriever.call_on_reads(&retrievers, &mut data, &mut repeats, &ver, ctx).map_err(|e| {
                serde::de::Error::custom(e)
            })?;
        }
        Ok(BaseStruct::new(ver, data, repeats))
    }
}
