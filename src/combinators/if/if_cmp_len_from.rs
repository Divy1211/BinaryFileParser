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
pub struct IfCmpLenFrom {
    target: usize,
    source: usize,
    ord: Vec<Ordering>,
    com: Box<CombinatorType>,
}

impl IfCmpLenFrom {
    pub fn new(target: usize, source: usize, ord: Vec<Ordering>, com: CombinatorType) -> Self {
        IfCmpLenFrom {
            target,
            source,
            ord,
            com: Box::new(com),
        }
    }
}

impl Combinator for IfCmpLenFrom {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        check_initialized(self.target, retrievers, data)?;
        check_initialized(self.source, retrievers, data)?;

        let Ok(target) = BfpList::try_from(get(self.target, retrievers, data, ver)?) else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpLenFrom: '{}' cannot be interpreted as a list", retrievers[self.target].name
            )))
        };
        let Ok(source) = get(self.source, retrievers, data, ver)?.try_into() else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpLenFrom: '{}' cannot be interpreted as an integer", retrievers[self.source].name
            )))
        };
        
        let ord = target.len().cmp(&source);
        
        if self.ord.contains(&ord) {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
