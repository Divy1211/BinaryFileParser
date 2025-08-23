use std::cmp::Ordering;

use pyo3::exceptions::{PyTypeError};
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
pub struct IfCmpKey {
    key: String,
    source: Vec<usize>,
    ord: Vec<Ordering>,
    coms: Vec<CombinatorType>,
}

impl IfCmpKey {
    pub fn new(key: &String, source: &Vec<usize>, ord: &Vec<Ordering>, coms: Vec<CombinatorType>) -> Self {
        IfCmpKey {
            key: key.clone(),
            source: source.clone(),
            ord: ord.clone(),
            coms,
        }
    }
}

impl Combinator for IfCmpKey {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let target = ctx.get(&self.key)?;
        let (source_name, source) = get_rec(&self.source, retrievers, data, ver)?;

        let Some(ord) = target.partial_cmp(&source) else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpKey: cannot compare Context key '{}' and '{}'",
                self.key,
                source_name,
            )));
        };
        
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
