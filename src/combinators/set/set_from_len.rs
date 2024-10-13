use pyo3::exceptions::{PyTypeError};
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::utils::{check_initialized, get};
use crate::retrievers::retriever::{RetState, Retriever};
use crate::types::bfp_list::BfpList;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub struct SetFromLen {
    target: usize,
    source: usize,
}

impl SetFromLen {
    pub fn new(target: usize, source: usize) -> Self {
        SetFromLen { target, source }
    }
}

impl Combinator for SetFromLen {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        check_initialized(self.target, retrievers, data)?;
        check_initialized(self.source, retrievers, data)?;

        let source = match BfpList::try_from(get(self.source, retrievers, data, ver)?) {
            Ok(ls) => { ls.len() }
            Err(_) => {
                return Err(PyTypeError::new_err(format!(
                    "SetFromLen: '{}' cannot be interpreted as a list", retrievers[self.source].name
                )))
            }
        };

        let target = &retrievers[self.target];
        let source = target.data_type.to_parseable_from_usize(source);
        
        data[self.target] = match (target.state(&repeats), source) {
            (RetState::List, _) | (_, None) => {
                return Err(PyTypeError::new_err(format!(
                    "SetTo: '{}' cannot be set to an integer", target.name
                )))
            }
            (_, source) => { source }
        };
        Ok(())
    }
}
