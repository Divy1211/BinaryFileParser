use std::cmp::Ordering;

use pyo3::exceptions::{PyTypeError};
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::utils::{check_initialized, get};
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub struct IfCmpFrom {
    target: usize,
    source: usize,
    ord: Vec<Ordering>,
    com: Box<CombinatorType>,
}

impl IfCmpFrom {
    pub fn new(target: usize, source: usize, ord: Vec<Ordering>, com: CombinatorType) -> Self {
        IfCmpFrom {
            target,
            source,
            ord,
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
        check_initialized(self.target, retrievers, data)?;
        check_initialized(self.source, retrievers, data)?;

        let target = get(self.target, retrievers, data, ver)?;
        let source = get(self.source, retrievers, data, ver)?;

        let Some(ord) = target.partial_cmp(&source) else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpFrom: cannot compare '{}' and '{}'",
                retrievers[self.target].name,
                retrievers[self.source].name,
            )));
        };
        
        if self.ord.contains(&ord) {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
