use std::cmp::Ordering;

use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::utils::{check_initialized, get};
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub struct IfCmpTo {
    target: usize,
    source: ParseableType,
    ord: Vec<Ordering>,
    com: Box<CombinatorType>,
}

impl IfCmpTo {
    pub fn new(target: usize, source: ParseableType, ord: Vec<Ordering>, com: CombinatorType) -> Self {
        IfCmpTo {
            target,
            source,
            ord,
            com: Box::new(com),
        }
    }
}

impl Combinator for IfCmpTo {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        check_initialized(self.target, retrievers, data)?;

        let target = get(self.target, retrievers, data, ver)?;

        let ord = target.partial_cmp(&self.source).expect("infallible");
        
        if self.ord.contains(&ord) {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
