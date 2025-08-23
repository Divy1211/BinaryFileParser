use std::cmp::Ordering;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::retrievers::retriever::Retriever;
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct IfCmpKeyTo {
    key: String,
    source: ParseableType,
    ord: Vec<Ordering>,
    coms: Vec<CombinatorType>,
}

impl IfCmpKeyTo {
    pub fn new(key: &String, source: &ParseableType, ord: &Vec<Ordering>, coms: Vec<CombinatorType>) -> Self {
        IfCmpKeyTo {
            key: key.clone(),
            source: source.clone(),
            ord: ord.clone(),
            coms,
        }
    }
}

impl Combinator for IfCmpKeyTo {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let target = ctx.get(&self.key)?;

        let Some(ord) = target.partial_cmp(&self.source) else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpKeyTo: Context key '{}' is not a number",
                self.key,
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
