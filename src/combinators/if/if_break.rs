use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::retrievers::retriever::Retriever;
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct IfBreak;

impl Combinator for IfBreak {
    fn run(
        &self,
        _retrievers: &Vec<Retriever>,
        _data: &mut Vec<Option<ParseableType>>,
        _repeats: &mut Vec<Option<isize>>,
        _ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        ctx.break_if();
        Ok(())
    }
}
