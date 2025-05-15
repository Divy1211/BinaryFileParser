use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::get::Get;
use crate::retrievers::retriever::Retriever;
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct SetRepeatBy {
    target: usize,
    source: Get,
}

impl SetRepeatBy {
    pub fn new(target: usize, source: Get) -> Self {
        SetRepeatBy {
            target,
            source,
        }
    }
}

impl Combinator for SetRepeatBy {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let source = self.source.eval(retrievers, data, repeats, ver, ctx)? as isize;
        
        if source < -2 {
            return Err(PyValueError::new_err(format!(
                "SetRepeatBy: Attempting to set repeat of '{}' to '{}', which is less than -2",
                retrievers[self.target].name, source
            )));
        }
        
        repeats[self.target] = Some(source);
        Ok(())
    }
}
