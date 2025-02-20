use std::cmp::Ordering;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::utils::{get_rec};
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct IfCmpLenTo {
    target: Vec<usize>,
    source: usize,
    ord: Vec<Ordering>,
    com: Box<CombinatorType>,
}

impl IfCmpLenTo {
    pub fn new(target: &Vec<usize>, source: usize, ord: &Vec<Ordering>, com: CombinatorType) -> Self {
        IfCmpLenTo {
            target: target.clone(),
            source,
            ord: ord.clone(),
            com: Box::new(com),
        }
    }
}

impl Combinator for IfCmpLenTo {
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
                "IfCmpLenTo: '{}' cannot be interpreted as a list", target_name
            )))
        };

        let ord = target.cmp(&self.source);
        
        if self.ord.contains(&ord) {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
