use std::cmp::Ordering;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::utils::{check_initialized, get};
use crate::retrievers::retriever::Retriever;
use crate::types::bfp_list::BfpList;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub struct IfCmpLenTo {
    target: usize,
    source: usize,
    ord: Vec<Ordering>,
    com: Box<CombinatorType>,
}

impl IfCmpLenTo {
    pub fn new(target: usize, source: usize, ord: Vec<Ordering>, com: CombinatorType) -> Self {
        IfCmpLenTo {
            target,
            source,
            ord,
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
        check_initialized(self.target, retrievers, data)?;

        let Ok(target) = BfpList::try_from(get(self.target, retrievers, data, ver)?) else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpLenTo: '{}' cannot be interpreted as a list", retrievers[self.target].name
            )))
        };

        let ord = target.len().cmp(&self.source);
        
        if self.ord.contains(&ord) {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
