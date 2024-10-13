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
    of: usize,
    from: usize,
}

impl SetFromLen {
    pub fn new(of: usize, from: usize) -> Self {
        SetFromLen { of, from, }
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
        check_initialized(self.of, retrievers, data)?;
        check_initialized(self.from, retrievers, data)?;

        let to = match BfpList::try_from(get(self.from, retrievers, data, ver)?) {
            Ok(ls) => { ls.len() }
            Err(_) => {
                return Err(PyTypeError::new_err(format!(
                    "SetFromLen: '{}' cannot be interpreted as a list", retrievers[self.from].name
                )))
            }
        };

        let of = &retrievers[self.of];
        let to = of.data_type.to_parseable_from_usize(to);
        
        data[self.of] = match (of.state(&repeats), to) {
            (RetState::List, _) | (_, None) => {
                return Err(PyTypeError::new_err(format!(
                    "SetTo: '{}' cannot be set to an integer", of.name
                )))
            }
            (_, to) => { to }
        };
        Ok(())
    }
}
