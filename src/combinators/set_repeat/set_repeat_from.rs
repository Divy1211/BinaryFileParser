use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::utils::{get_rec};
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct SetRepeatFrom {
    target: usize,
    source: Vec<usize>,
}

impl SetRepeatFrom {
    pub fn new(target: usize, source: Vec<usize>) -> Self {
        SetRepeatFrom {
            target,
            source,
        }
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
        let (source_name, source) = get_rec(&self.source, retrievers, data, ver)?;

        let Ok(source) = (&source).try_into() else {
            return Err(PyTypeError::new_err(format!(
                "SetRepeat: '{}' cannot be interpreted as an integer", source_name
            )))
        };

        if source < -2 {
            return Err(PyValueError::new_err(format!(
                "SetRepeatBy: Attempting to set repeat of '{}' to '{}' from '{}', which is less than -2",
                retrievers[self.target].name, source, source_name
            )));
        }
        
        repeats[self.target] = Some(source);
        Ok(())
    }
}
