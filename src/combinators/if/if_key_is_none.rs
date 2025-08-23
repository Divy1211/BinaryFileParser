use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::retrievers::retriever::Retriever;
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct IfKeyIsNone {
    key: String,
    coms: Vec<CombinatorType>,
    not: bool,
}

impl IfKeyIsNone {
    pub fn new(key: &String, coms: Vec<CombinatorType>, not: bool) -> Self {
        IfKeyIsNone {
            key: key.clone(),
            coms,
            not,
        }
    }
}

impl Combinator for IfKeyIsNone {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let source = ctx.get(&self.key)?;
        
        ctx.enter_if();
        if (source == ParseableType::None) ^ self.not {
            for com in &self.coms {
                com.run(retrievers, data, repeats, ver, ctx)?;
            }
            ctx.run_if();
        }
        Ok(())
    }
}
