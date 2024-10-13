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
    of: usize,
    from: usize,
}

impl SetFrom {
    pub fn new(of: usize, from: usize) -> Self {
        SetFrom { of, from, }
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
        check_initialized(self.of, retrievers, data)?;
        check_initialized(self.from, retrievers, data)?;
        
        let to = get(self.from, retrievers, data, ver)?;
        
        let of = &retrievers[self.of];

        data[self.of] = Some(match of.state(&repeats) {
            RetState::List if !to.is_ls_of(&of.data_type) => {
                return Err(PyTypeError::new_err(format!(
                    "SetTo: Unable to set '{}' from {}", of.name, retrievers[self.from].name
                )))
            }
            _ => { to.clone() }
        });
        Ok(())
    }
}
