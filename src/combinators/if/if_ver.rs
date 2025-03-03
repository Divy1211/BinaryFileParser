use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct IfVer {
    min_ver: Version,
    max_ver: Version,
    com: Box<CombinatorType>,
}

impl IfVer {
    pub fn new(min_ver: &Version, max_ver: &Version, com: CombinatorType) -> Self {
        IfVer {
            min_ver: min_ver.clone(),
            max_ver: max_ver.clone(),
            com: Box::new(com),
        }
    }
}

impl Combinator for IfVer {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version
    ) -> PyResult<()> {
        if self.min_ver <= *ver && *ver <= self.max_ver {
            self.com.run(retrievers, data, repeats, ver)?;
        }
        Ok(())
    }
}
