use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::retrievers::retriever::{RetState, Retriever};
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub struct SetTo {
    target: usize,
    source: ParseableType,
}

impl SetTo {
    pub fn new(target: usize, source: ParseableType) -> Self {
        SetTo { target, source, }
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
        let target = &retrievers[self.target];
        
        data[self.target] = Some(match target.state(&repeats) {
            RetState::List if !self.source.is_ls_of(&target.data_type) => {
                return Err(PyTypeError::new_err(format!(
                    "SetTo: Unable to set '{}' from value of incorrect type", target.name
                )))
            }
            _ => { self.source.clone() }
        });
        Ok(())
    }
}
