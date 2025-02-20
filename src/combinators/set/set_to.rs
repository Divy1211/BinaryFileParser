use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::utils::set_rec;
use crate::retrievers::retriever::{Retriever};
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct SetTo {
    target: Vec<usize>,
    source: ParseableType,
}

impl SetTo {
    pub fn new(target: &Vec<usize>, source: ParseableType) -> Self {
        SetTo {
            target: target.clone(),
            source
        }
    }
}

impl Combinator for SetTo {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        set_rec(&self.target, retrievers, data, repeats, ver, self.source.clone())
    }
}
