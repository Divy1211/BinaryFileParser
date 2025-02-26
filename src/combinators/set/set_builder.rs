use pyo3::prelude::*;
use pyo3::types::{PyTuple};

use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::get::Get;
use crate::combinators::set::set_by::SetBy;
use crate::combinators::set::set_from::SetFrom;
use crate::combinators::utils::idxes_from_tup;
use crate::combinators::set::set_from_len::SetFromLen;
use crate::combinators::set::set_to::SetTo;
use crate::types::bfp_type::BfpType;

#[pyclass(module = "bfp_rs.combinators")]
pub struct SetBuilder {
    target: Vec<usize>,
    target_data_type: BfpType,
    target_name: String,
}

#[pymethods]
impl SetBuilder {
    pub fn by(&self, mut from: Get) -> PyResult<CombinatorType> {
        from.make_contiguous();
        Ok(SetBy::new(
            &self.target,
            from,
            &self.target_data_type,
        ).into())
    }
    
    #[pyo3(signature = (*from), text_signature = "(*from: Retriever | int)")]
    pub fn from_(&self, from: &Bound<PyTuple>) -> PyResult<CombinatorType> {
        let (source, _source_data_type, _source_name) = idxes_from_tup(from)?;
        
        Ok(SetFrom::new(
            &self.target,
            source,
        ).into())
    }

    #[pyo3(signature = (*from), text_signature = "(*from: Retriever | int)")]
    pub fn from_len(&self, from: &Bound<'_, PyTuple>) -> PyResult<CombinatorType> {
        let (source, _source_data_type, _source_name) = idxes_from_tup(from)?;
        
        Ok(SetFromLen::new(
            &self.target,
            source,
            &self.target_data_type,
            &self.target_name,
        ).into())
    }

    pub fn to(&self, val: &Bound<PyAny>) -> PyResult<CombinatorType> {
        Ok(SetTo::new(
            &self.target,
            self.target_data_type.to_parseable(val)?, // todo: figure this out
        ).into())
    }
}

#[pyfunction(name = "set_")]
#[pyo3(signature = (*target), text_signature = "(*target: Retriever | int)")]
pub fn set(target: &Bound<PyTuple>) -> PyResult<SetBuilder> {
    let (target, target_data_type, target_name) = idxes_from_tup(target)?;

    Ok(SetBuilder {
        target,
        target_data_type,
        target_name,
    })
}
