use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::utils::{get_rec, set_rec};
use crate::retrievers::retriever::{Retriever};
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub struct SetFrom {
    target: Vec<usize>,
    source: Vec<usize>,
}

impl SetFrom {
    pub fn new(target: Vec<usize>, source: Vec<usize>) -> Self {
        SetFrom { target, source }
    }
}

impl Combinator for SetFrom {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        let source= get_rec(&self.source, retrievers, data, ver)?;
        set_rec(&self.target, retrievers, data, repeats, ver, source, false)
    }
}
