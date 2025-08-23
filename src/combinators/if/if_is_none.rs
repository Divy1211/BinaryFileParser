use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::utils::{get_rec};
use crate::retrievers::retriever::Retriever;
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct IfIsNone {
    source: Vec<usize>,
    coms: Vec<CombinatorType>,
    not: bool,
}

impl IfIsNone {
    pub fn new(source: &Vec<usize>, coms: Vec<CombinatorType>, not: bool) -> Self {
        IfIsNone {
            source: source.clone(),
            coms,
            not,
        }
    }
}

impl Combinator for IfIsNone {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let (_name, source) = get_rec(&self.source, retrievers, data, ver)?;
        
        ctx.enter_if();
        if (source == ParseableType::None) ^ self.not {
            for com in &self.coms {
                com.run(retrievers, data, repeats, ver, ctx)?;
            }
            ctx.run_if();
        }
        Ok(())
    }
}
