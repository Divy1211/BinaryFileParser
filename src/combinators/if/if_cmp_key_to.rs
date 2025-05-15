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
    com: Box<CombinatorType>,
}

impl IfCmpKeyTo {
    pub fn new(key: &String, source: &ParseableType, ord: &Vec<Ordering>, com: CombinatorType) -> Self {
        IfCmpKeyTo {
            key: key.clone(),
            source: source.clone(),
            ord: ord.clone(),
            com: Box::new(com),
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
        
        if self.ord.contains(&ord) {
            self.com.run(retrievers, data, repeats, ver, ctx)?;
        }
        Ok(())
    }
}
