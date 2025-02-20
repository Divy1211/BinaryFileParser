use std::cmp::Ordering;

use pyo3::exceptions::{PyTypeError};
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::utils::{get_rec};
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct IfCmpFrom {
    target: Vec<usize>,
    source: Vec<usize>,
    ord: Vec<Ordering>,
    com: Box<CombinatorType>,
}

impl IfCmpFrom {
    pub fn new(target: &Vec<usize>, source: &Vec<usize>, ord: &Vec<Ordering>, com: CombinatorType) -> Self {
        IfCmpFrom {
            target: target.clone(),
            source: source.clone(),
            ord: ord.clone(),
            com: Box::new(com),
        }
    }
}

impl Combinator for IfCmpFrom {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        let (target_name, target) = get_rec(&self.target, retrievers, data, ver)?;
        let (source_name, source) = get_rec(&self.source, retrievers, data, ver)?;

        let Some(ord) = target.partial_cmp(&source) else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpFrom: cannot compare '{}' and '{}'",
                target_name,
                source_name,
            )));
        };
        
        if self.ord.contains(&ord) {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
