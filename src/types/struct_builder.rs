use pyo3::{intern, pyclass, Bound, PyObject, PyResult};
use pyo3::types::{PyBytes, PyString, PyType};
use pyo3::prelude::{PyAnyMethods, PyTypeMethods};

use crate::errors::compression_error::CompressionError;
use crate::errors::version_error::VersionError;
use crate::retrievers::retriever::Retriever;
use crate::retrievers::retriever_combiner::RetrieverCombiner;
use crate::retrievers::retriever_ref::RetrieverRef;
use crate::types::byte_stream::ByteStream;
use crate::types::r#struct::{Struct, StructRaw};

#[pyclass(module = "bfp_rs")]
#[derive(Debug, Clone)]
pub struct StructBuilder {
    retrievers: Vec<Retriever>,
    combiners: Vec<RetrieverCombiner>,
    refs: Vec<RetrieverRef>,
}

impl StructBuilder {
    pub fn new() -> Self {
        Self {
            retrievers: Vec::new(),
            combiners: Vec::new(),
            refs: Vec::new(),
        }
    }

    pub fn add_ret(&mut self, retriever: &Bound<Retriever>) -> PyResult<usize> {
        let mut retriever = retriever.extract::<Retriever>()?;
        let idx = self.retrievers.len();
        retriever.idx = idx;
        self.retrievers.push(retriever);
        Ok(idx)
    }

    pub fn add_comb(&mut self, combiner: &Bound<RetrieverCombiner>) -> PyResult<()> {
        self.combiners.push(combiner.extract()?);
        Ok(())
    }

    pub fn add_ref(&mut self, ref_: &Bound<RetrieverRef>) -> PyResult<()> {
        self.refs.push(ref_.extract()?);
        Ok(())
    }
    
    pub fn get_struct(cls: &Bound<PyType>) -> PyResult<Struct> {
        let builder = cls
            .getattr(intern!(cls.py(), "__struct_builder__")).expect("always a BaseStruct subclass");

        if builder.is_none() {
            return cls
                .getattr(intern!(cls.py(), "__struct__")).expect("always a BaseStruct subclass")
                .extract();
        }
        
        let mut builder = builder.extract::<StructBuilder>().expect("infallible");
        
        let get_ver = get_if_impl(cls, intern!(cls.py(), "_get_version"));
        let compress = get_if_impl(cls, intern!(cls.py(), "_compress"));
        let decompress = get_if_impl(cls, intern!(cls.py(), "_decompress"));
        
        let mut is_compressed = true;
        for (i, retriever) in builder.retrievers.iter_mut().enumerate() {
            if i == 0 {
                is_compressed = retriever.remaining_compressed;
                retriever.remaining_compressed = false;
            }
            retriever.construct_fns(cls.py())?
        }

        cls.setattr(intern!(cls.py(), "__struct_builder__"), cls.py().None())?;
        
        let struct_ = Struct::from_raw( StructRaw {
            retrievers: builder.retrievers,
            combiners: builder.combiners,
            refs: builder.refs,

            py_type: cls.extract()?,
            fully_qualified_name: cls.fully_qualified_name()?.to_string(),

            is_compressed,
            
            get_ver,
            compress,
            decompress,
        });
        
        cls.setattr(intern!(cls.py(), "__struct__"), Bound::new(cls.py(), struct_.clone())?)?;
        
        Ok(struct_)
    }
}

fn get_if_impl(cls: &Bound<PyType>, attr: &Bound<PyString>) -> Option<PyObject> {
    let py = cls.py();
    let obj = cls.getattr(attr).expect("always a BaseStruct subclass");
    if attr == "_get_version" {
        match obj.call1((ByteStream::empty(),)) {
            Err(err) if err.is_instance_of::<VersionError>(py) => None,
            _ => Some(obj.unbind())
        }
    } else {
        match obj.call1((PyBytes::new_bound(py, &[]),)) {
            Err(err) if err.is_instance_of::<CompressionError>(py) => None,
            _ => Some(obj.unbind())
        }
    }
}
