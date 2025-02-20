use pyo3::exceptions::{PyTypeError};
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::utils::{get_rec};
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct SetRepeatFromLen {
    target: usize,
    source: Vec<usize>,
}

impl SetRepeatFromLen {
    pub fn new(target: usize, source: Vec<usize>) -> Self {
        SetRepeatFromLen {
            target,
            source,
        }
    }
}

impl Combinator for SetRepeatFromLen {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        let (source_name, source) = get_rec(&self.source, retrievers, data, ver)?;
        
        let Some(source) = source.try_len() else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpLenFrom: '{}' cannot be interpreted as a list", source_name
            )))
        };
        
        repeats[self.target] = Some(source as isize);
        Ok(())
    }
}
