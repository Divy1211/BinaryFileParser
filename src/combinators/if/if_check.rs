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
    source: usize,
    com: Box<CombinatorType>,
}

impl IfCheck {
    pub fn new(source: usize, com: CombinatorType) -> Self {
        IfCheck {
            source,
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
        check_initialized(self.source, retrievers, data)?;

        let Ok(source_val) = get(self.source, retrievers, data, ver)?.try_into() else {
            return Err(PyTypeError::new_err(format!(
                "IfCheck: '{}' cannot be interpreted as a boolean", retrievers[self.source].name
            )))
        };
        
        if source_val {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
