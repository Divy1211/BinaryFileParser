use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::utils::{get_rec};
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct IfIsNone {
    source: Vec<usize>,
    com: Box<CombinatorType>,
    not: bool,
}

impl IfIsNone {
    pub fn new(source: &Vec<usize>, com: CombinatorType, not: bool) -> Self {
        IfIsNone {
            source: source.clone(),
            com: Box::new(com),
            not,
        }
    }
}

impl Combinator for IfIsNone {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        let (_name, source) = get_rec(&self.source, retrievers, data, ver)?;
        
        if (source == ParseableType::None) ^ self.not {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
