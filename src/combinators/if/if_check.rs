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
pub struct IfCheck {
    source: Vec<usize>,
    com: Box<CombinatorType>,
    not: bool,
}

impl IfCheck {
    pub fn new(source: &Vec<usize>, com: CombinatorType, not: bool) -> Self {
        IfCheck {
            source: source.clone(),
            com: Box::new(com),
            not,
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
        let (name, source) = get_rec(&self.source, retrievers, data, ver)?;
        
        let Ok(source_val): Result<bool, _> = (&source).try_into() else {
            return Err(PyTypeError::new_err(format!(
                "IfCheck: '{}' cannot be interpreted as a boolean", name
            )))
        };
        
        if source_val ^ self.not {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
