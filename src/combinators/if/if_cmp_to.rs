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
    val1: usize,
    val2: ParseableType,
    ord: Vec<Ordering>,
    com: Box<CombinatorType>,
}

impl IfCmpTo {
    pub fn new(val1: usize, val2: ParseableType, ord: Vec<Ordering>, com: CombinatorType) -> Self {
        IfCmpTo {
            val1,
            val2,
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
        check_initialized(self.val1, retrievers, data)?;

        let val1 = get(self.val1, retrievers, data, ver)?;

        let ord = val1.partial_cmp(&self.val2).expect("infallible");
        
        if self.ord.contains(&ord) {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
