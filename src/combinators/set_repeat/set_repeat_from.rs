use pyo3::exceptions::{PyTypeError};
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::utils::{check_initialized, get};
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub struct SetRepeatFrom {
    target: usize,
    source: usize,
}

impl SetRepeatFrom {
    pub fn new(target: usize, source: usize) -> Self {
        SetRepeatFrom { target, source }
    }
}

impl Combinator for SetRepeatFrom {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        check_initialized(self.source, retrievers, data)?;
        
        let source = get(self.source, retrievers, data, ver)?;
        
        let Ok(source) = source.try_into() else {
            return Err(PyTypeError::new_err(format!(
                "SetRepeat: '{}' cannot be interpreted as an integer", retrievers[self.source].name
            )))
        };
        
        repeats[self.target] = Some(source);
        Ok(())
    }
}
