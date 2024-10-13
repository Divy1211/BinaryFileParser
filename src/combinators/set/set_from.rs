use pyo3::exceptions::{PyTypeError};
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::utils::{check_initialized, get};
use crate::retrievers::retriever::{RetState, Retriever};
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub struct SetFrom {
    target: usize,
    source: usize,
}

impl SetFrom {
    pub fn new(target: usize, source: usize) -> Self {
        SetFrom { target, source }
    }
}

impl Combinator for SetFrom {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        check_initialized(self.target, retrievers, data)?;
        check_initialized(self.source, retrievers, data)?;

        let source = get(self.source, retrievers, data, ver)?;

        let target = &retrievers[self.target];

        data[self.target] = Some(match target.state(&repeats) {
            RetState::List if !source.is_ls_of(&target.data_type) => {
                return Err(PyTypeError::new_err(format!(
                    "SetTo: Unable to set '{}' from {}", target.name, retrievers[self.source].name
                )))
            }
            _ => { source.clone() }
        });
        Ok(())
    }
}
