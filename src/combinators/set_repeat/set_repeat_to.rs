use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub struct SetRepeatTo {
    target: usize,
    source: isize,
}

impl SetRepeatTo {
    pub fn new(target: usize, source: isize) -> Self {
        SetRepeatTo { target, source, }
    }
}

impl Combinator for SetRepeatTo {
    fn run(
        &self,
        _retrievers: &Vec<Retriever>,
        _data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        _ver: &Version
    ) -> PyResult<()> {
        repeats[self.target] = Some(self.source);
        Ok(())
    }
}
