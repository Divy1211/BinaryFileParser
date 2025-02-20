use std::cmp::Ordering;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::get::Get;
use crate::combinators::utils::{get_rec};
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct IfCmpLenBy {
    target: Vec<usize>,
    source: Get,
    ord: Vec<Ordering>,
    com: Box<CombinatorType>,
}

impl IfCmpLenBy {
    pub fn new(target: &Vec<usize>, source: &Get, ord: &Vec<Ordering>, com: CombinatorType) -> Self {
        IfCmpLenBy {
            target: target.clone(),
            source: source.clone(),
            ord: ord.clone(),
            com: Box::new(com),
        }
    }
}

impl Combinator for IfCmpLenBy {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        let (target_name, target) = get_rec(&self.target, retrievers, data, ver)?;

        let Some(target) = target.try_len() else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpLenBy: '{}' cannot be interpreted as a list", target_name
            )))
        };
        let target = target as i128;
        
        let source = self.source.eval(retrievers, data, repeats, ver)?;
        let ord = target.cmp(&source);
        
        if self.ord.contains(&ord) {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
