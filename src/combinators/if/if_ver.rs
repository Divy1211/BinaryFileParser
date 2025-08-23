use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::retrievers::retriever::Retriever;
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct IfVer {
    min_ver: Version,
    max_ver: Version,
    coms: Vec<CombinatorType>,
}

impl IfVer {
    pub fn new(min_ver: &Version, max_ver: &Version, coms: Vec<CombinatorType>) -> Self {
        IfVer {
            min_ver: min_ver.clone(),
            max_ver: max_ver.clone(),
            coms,
        }
    }
}

impl Combinator for IfVer {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        ctx.enter_if();
        if self.min_ver <= *ver && *ver <= self.max_ver {
            for com in &self.coms {
                com.run(retrievers, data, repeats, ver, ctx)?;
            }
            ctx.run_if();
        }
        Ok(())
    }
}
