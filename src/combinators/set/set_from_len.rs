use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::combinators::combinator::Combinator;
use crate::combinators::utils::{get_rec, set_rec};
use crate::retrievers::retriever::Retriever;
use crate::types::bfp_type::BfpType;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub struct SetFromLen {
    target: Vec<usize>,
    source: Vec<usize>,
    target_data_type: BfpType,
    target_name: String,
}

impl SetFromLen {
    pub fn new(target: &Vec<usize>, source: Vec<usize>, target_data_type: &BfpType, target_name: &str) -> Self {
        SetFromLen {
            target: target.clone(),
            source,
            target_data_type: target_data_type.clone(),
            target_name: target_name.to_string(),
        }
    }
}

impl Combinator for SetFromLen {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
    ) -> PyResult<()> {
        let (name, source) = get_rec(&self.source, retrievers, data, ver)?;

        let source = match source.try_len() {
            Some(len) => { len }
            None => {
                return Err(PyTypeError::new_err(format!(
                    "SetFromLen: '{}' cannot be interpreted as a list", name
                )))
            }
        };

        let Some(source) = self.target_data_type.to_parseable_from_int(source as i128) else {
            return Err(PyTypeError::new_err(format!(
                "SetFromLen: '{}' cannot be set to an int", self.target_name
            )))
        };

        set_rec(&self.target, retrievers, data, repeats, ver, source)
    }
}
