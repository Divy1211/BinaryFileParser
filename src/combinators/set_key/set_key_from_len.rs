use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::utils::{get_rec};
use crate::retrievers::retriever::Retriever;
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct SetKeyFromLen {
    key: String,
    source: Vec<usize>,
}

impl SetKeyFromLen {
    pub fn new(key: &String, source: Vec<usize>) -> Self {
        SetKeyFromLen {
            key: key.clone(),
            source,
        }
    }
}

impl Combinator for SetKeyFromLen {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        _repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let (name, source) = get_rec(&self.source, retrievers, data, ver)?;

        let source = match source.try_len() {
            Some(len) => { len }
            None => {
                return Err(PyTypeError::new_err(format!(
                    "SetFromLen: '{}' cannot be interpreted as a list", name
                )))
            }
        };
        let source = ParseableType::Int128(source as i128);
        ctx.set(&self.key, source);
        Ok(())
    }
}
