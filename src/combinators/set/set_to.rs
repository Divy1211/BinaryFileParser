use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::retrievers::retriever::{RetState, Retriever};
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub struct SetTo {
    of: usize,
    to: ParseableType,
}

impl SetTo {
    pub fn new(of: usize, to: ParseableType) -> Self {
        SetTo { of, to, }
    }
}

impl Combinator for SetTo {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        _ver: &Version
    ) -> PyResult<()> {
        let of = &retrievers[self.of];
        
        data[self.of] = Some(match of.state(&repeats) {
            RetState::List if !self.to.is_ls_of(&of.data_type) => {
                return Err(PyTypeError::new_err(format!(
                    "SetTo: Unable to set '{}' from value of incorrect type", of.name
                )))
            }
            _ => { self.to.clone() }
        });
        Ok(())
    }
}
