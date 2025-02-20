use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::get::Get;
use crate::combinators::utils::{set_rec};
use crate::retrievers::retriever::{Retriever};
use crate::types::bfp_type::BfpType;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct SetBy {
    target: Vec<usize>,
    target_data_type: BfpType,
    source: Get,
}

impl SetBy {
    pub fn new(target: &Vec<usize>, source: Get, target_data_type: &BfpType) -> Self {
        SetBy {
            target: target.clone(),
            source,
            target_data_type: target_data_type.clone()
        }
    }
}

impl Combinator for SetBy {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        let source = self.source.eval(retrievers, data, repeats, ver)?;
        let source = self.target_data_type.to_parseable_from_int(source).expect("Infallible");
        set_rec(&self.target, retrievers, data, repeats, ver, source)
    }
}
