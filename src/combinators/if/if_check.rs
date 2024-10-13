use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::utils::{check_initialized, get};
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub struct IfCheck {
    val: usize,
    com: Box<CombinatorType>,
}

impl IfCheck {
    pub fn new(val: usize, com: CombinatorType) -> Self {
        IfCheck {
            val,
            com: Box::new(com),
        }
    }
}

impl Combinator for IfCheck {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        check_initialized(self.val, retrievers, data)?;

        let Ok(val) = get(self.val, retrievers, data, ver)?.try_into() else {
            return Err(PyTypeError::new_err(format!(
                "IfCheck: '{}' cannot be interpreted as a boolean", retrievers[self.val].name
            )))
        };
        
        if val {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
