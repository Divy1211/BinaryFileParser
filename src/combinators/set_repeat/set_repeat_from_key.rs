use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::retrievers::retriever::Retriever;
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct SetRepeatFromKey {
    target: usize,
    key: String,
}

impl SetRepeatFromKey {
    pub fn new(target: usize, key: String) -> Self {
        SetRepeatFromKey {
            target,
            key,
        }
    }
}

impl Combinator for SetRepeatFromKey {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        _data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        _ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let source = ctx.get(&self.key)?;

        let Ok(source) = (&source).try_into() else {
            return Err(PyTypeError::new_err(format!(
                "SetRepeatFromKey: Context key '{}' cannot be interpreted as an integer", self.key
            )))
        };

        if source < -2 {
            return Err(PyValueError::new_err(format!(
                "SetRepeatFromKey: Attempting to set repeat of '{}' to '{}' from context key '{}', which is less than -2",
                retrievers[self.target].name, source, self.key
            )));
        }
        
        repeats[self.target] = Some(source);
        Ok(())
    }
}
