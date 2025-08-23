use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::get::Get;
use crate::retrievers::retriever::{Retriever};
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct SetKeyBy {
    key: String,
    source: Get,
}

impl SetKeyBy {
    pub fn new(key: &String, source: Get) -> Self {
        SetKeyBy {
            key: key.clone(),
            source,
        }
    }
}

impl Combinator for SetKeyBy {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let source = self.source.eval(retrievers, data, repeats, ver, ctx)?;
        let source = ParseableType::Int128(source);
        ctx.set(&self.key, source);
        Ok(())
    }
}
