use std::cmp::Ordering;

use pyo3::exceptions::{PyTypeError};
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
pub struct IfCmpBy {
    target: Vec<usize>,
    source: Get,
    ord: Vec<Ordering>,
    com: Box<CombinatorType>,
}

impl IfCmpBy {
    pub fn new(target: &Vec<usize>, source: &Get, ord: &Vec<Ordering>, com: CombinatorType) -> Self {
        IfCmpBy {
            target: target.clone(),
            source: source.clone(),
            ord: ord.clone(),
            com: Box::new(com),
        }
    }
}

impl Combinator for IfCmpBy {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        let (target_name, target) = get_rec(&self.target, retrievers, data, ver)?;

        let Some(target) = target.try_to_int() else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpBy: cannot interpret '{}' as an int",
                target_name,
            )));
        };
        let source = self.source.eval(retrievers, data, repeats, ver)?;
        
        if self.ord.contains(&target.cmp(&source)) {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
