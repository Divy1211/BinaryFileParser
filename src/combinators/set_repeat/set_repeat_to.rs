use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct SetRepeatTo {
    target: usize,
    source: isize,
}

impl SetRepeatTo {
    pub fn new(target: usize, source: isize) -> Self {
        SetRepeatTo { target, source }
    }
}

impl Combinator for SetRepeatTo {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        _data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        _ver: &Version
    ) -> PyResult<()> {
        if self.source < -2 {
            return Err(PyValueError::new_err(format!(
                "SetRepeatTo: Attempting to set repeat of '{}' to '{}', which is less than -2",
                retrievers[self.target].name, self.source
            )));
        }
        repeats[self.target] = Some(self.source);
        Ok(())
    }
}
