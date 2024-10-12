use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub struct SetRepeatTo {
    of: usize,
    to: isize,
}

impl SetRepeatTo {
    pub fn new(of: usize, to: isize) -> Self {
        SetRepeatTo { of, to, }
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
        repeats[self.of] = Some(self.to);
        Ok(())
    }
}
