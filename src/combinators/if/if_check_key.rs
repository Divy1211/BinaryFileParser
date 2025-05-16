use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::combinator_type::CombinatorType;
use crate::retrievers::retriever::Retriever;
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct IfCheckKey {
    key: String,
    coms: Vec<CombinatorType>,
    not: bool,
}

impl IfCheckKey {
    pub fn new(key: &String, coms: Vec<CombinatorType>, not: bool) -> Self {
        IfCheckKey {
            key: key.clone(),
            coms,
            not,
        }
    }
}

impl Combinator for IfCheckKey {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        let source = ctx.get(&self.key)?;
        
        let Ok(source_val): Result<bool, _> = (&source).try_into() else {
            return Err(PyTypeError::new_err(format!(
                "IfCheckKey: Context key '{}' cannot be interpreted as a boolean", self.key
            )))
        };

        ctx.enter_if();
        if source_val ^ self.not {
            for com in &self.coms {
                com.run(retrievers, data, repeats, ver, ctx)?;
            }
            ctx.run_if();
        }
        Ok(())
    }
}
