use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::retrievers::retriever::Retriever;
use crate::types::context::{Context, IfTracker};
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct IfElse {
    coms: Vec<CombinatorType>,
}

impl IfElse {
    pub fn new(coms: Vec<CombinatorType>) -> Self {
        IfElse { coms }
    }
}

impl Combinator for IfElse {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        for com in &self.coms {
            ctx.if_tracker = Some(IfTracker::new());
            com.run(retrievers, data, repeats, ver, ctx)?;
            if ctx.do_break() {
                break;
            }
        }
        ctx.if_tracker = None;
        Ok(())
    }
}
