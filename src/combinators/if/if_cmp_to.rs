use std::cmp::Ordering;

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
pub struct IfCmpTo {
    target: Vec<usize>,
    source: ParseableType,
    ord: Vec<Ordering>,
    coms: Vec<CombinatorType>,
}

impl IfCmpTo {
    pub fn new(target: &Vec<usize>, source: &ParseableType, ord: &Vec<Ordering>, coms: Vec<CombinatorType>) -> Self {
        IfCmpTo {
            target: target.clone(),
            source: source.clone(),
            ord: ord.clone(),
            coms,
        }
    }
}

impl Combinator for IfCmpTo {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let (_target_name, target) = get_rec(&self.target, retrievers, data, ver)?;

        let ord = target.partial_cmp(&self.source).expect("infallible");

        ctx.enter_if();
        if self.ord.contains(&ord) {
            for com in &self.coms {
                com.run(retrievers, data, repeats, ver, ctx)?;
            }
            ctx.run_if();
        }
        Ok(())
    }
}
