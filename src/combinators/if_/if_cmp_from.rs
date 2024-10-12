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
    val1: usize,
    val2: usize,
    ord: Vec<Ordering>,
    com: Box<CombinatorType>,
}

impl IfCmpFrom {
    pub fn new(val1: usize, val2: usize, ord: Vec<Ordering>, com: CombinatorType) -> Self {
        IfCmpFrom {
            val1,
            val2,
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
        check_initialized(self.val1, retrievers, data)?;
        check_initialized(self.val2, retrievers, data)?;

        let val1 = get(self.val1, retrievers, data, ver)?;
        let val2 = get(self.val2, retrievers, data, ver)?;

        let Some(ord) = val1.partial_cmp(&val2) else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpFrom: cannot compare '{}' and '{}'",
                retrievers[self.val1].name,
                retrievers[self.val2].name,
            )));
        };
        
        if self.ord.contains(&ord) {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
