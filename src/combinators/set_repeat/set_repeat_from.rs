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
    of: usize,
    from: usize,
}

impl SetRepeatFrom {
    pub fn new(of: usize, from: usize) -> Self {
        SetRepeatFrom { of, from, }
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
        check_initialized(self.from, retrievers, data)?;
        let repeat = get(self.from, retrievers, data, ver)?;
        
        let Ok(repeat) = repeat.try_into() else {
            return Err(PyTypeError::new_err(format!(
                "SetRepeat: '{}' cannot be interpreted as an integer", retrievers[self.from].name
            )))
        };
        
        repeats[self.of] = Some(repeat);
        Ok(())
    }
}
