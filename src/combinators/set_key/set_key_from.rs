use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::utils::{get_rec};
use crate::retrievers::retriever::{Retriever};
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct SetKeyFrom {
    key: String,
    source: Vec<usize>,
}

impl SetKeyFrom {
    pub fn new(key: &String, source: Vec<usize>) -> Self {
        SetKeyFrom {
            key: key.clone(),
            source,
        }
    }
}

impl Combinator for SetKeyFrom {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        _repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let (_name, source) = get_rec(&self.source, retrievers, data, ver)?;
        ctx.set(&self.key, source);
        Ok(())
    }
}
