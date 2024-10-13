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
    val1: usize,
    val2: usize,
    ord: Vec<Ordering>,
    com: Box<CombinatorType>,
}

impl IfCmpLenFrom {
    pub fn new(val1: usize, val2: usize, ord: Vec<Ordering>, com: CombinatorType) -> Self {
        IfCmpLenFrom {
            val1,
            val2,
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
        check_initialized(self.val1, retrievers, data)?;
        check_initialized(self.val2, retrievers, data)?;

        let Ok(val1) = BfpList::try_from(get(self.val1, retrievers, data, ver)?) else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpLenFrom: '{}' cannot be interpreted as a list", retrievers[self.val1].name
            )))
        };
        let Ok(val2) = get(self.val2, retrievers, data, ver)?.try_into() else {
            return Err(PyTypeError::new_err(format!(
                "IfCmpLenFrom: '{}' cannot be interpreted as an integer", retrievers[self.val2].name
            )))
        };
        
        let ord = val1.len().cmp(&val2);
        
        if self.ord.contains(&ord) {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
