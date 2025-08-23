use std::cmp::Ordering;
use pyo3::exceptions::PyTypeError;
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
pub struct IfCmpLenTo {
    target: Vec<usize>,
    source: usize,
    ord: Vec<Ordering>,
    coms: Vec<CombinatorType>,
}

impl IfCmpLenTo {
    pub fn new(target: &Vec<usize>, source: usize, ord: &Vec<Ordering>, coms: Vec<CombinatorType>) -> Self {
        IfCmpLenTo {
            target: target.clone(),
            source,
            ord: ord.clone(),
            coms,
        }
    }
}

impl Combinator for IfCmpLenTo {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let (target_name, target) = get_rec(&self.target, retrievers, data, ver)?;


        let Some(target) = target.try_len() else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpLenTo: '{}' cannot be interpreted as a list", target_name
            )))
        };

        let ord = target.cmp(&self.source);

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
