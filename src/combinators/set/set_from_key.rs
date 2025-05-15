use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::utils::{set_rec};
use crate::retrievers::retriever::{Retriever};
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct SetFromKey {
    target: Vec<usize>,
    key: String,
}

impl SetFromKey {
    pub fn new(target: &Vec<usize>, key: String) -> Self {
        SetFromKey {
            target: target.clone(),
            key,
        }
    }
}

impl Combinator for SetFromKey {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let source = ctx.get(&self.key)?;
        set_rec(&self.target, retrievers, data, repeats, ver, source)
    }
}
