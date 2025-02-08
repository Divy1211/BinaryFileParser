use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyTuple};
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::set::set_from::SetFrom;
use crate::combinators::utils::idxes_from_tup;
// use crate::combinators::set::set_from_len::SetFromLen;
// use crate::combinators::set::set_to::SetTo;
use crate::types::bfp_type::BfpType;

#[pyclass]
pub struct SetBuilder {
    target: Vec<usize>,
    target_data_type: BfpType
}

#[pymethods]
impl SetBuilder {
    #[pyo3(signature = (*from), text_signature = "(*from: Retriever | int)")]
    pub fn from_(&self, from: &Bound<'_, PyTuple>) -> PyResult<CombinatorType> {
        let (source, _data_type) = idxes_from_tup(from)?;
        Ok(SetFrom::new(
            self.target.clone(),
            source,
        ).into())
    }

    // pub fn from_len(&self, from: Bound<Retriever>) -> CombinatorType {
    //     SetFromLen::new(
    //         self.target,
    //         from.borrow().idx,
    //     ).into()
    // }
    // 
    // pub fn to(&self, val: &Bound<PyAny>) -> PyResult<CombinatorType> {
    //     Ok(
    //         SetTo::new(
    //             self.target,
    //             self.target_data_type.to_parseable(val)?, // todo: figure this out
    //         ).into()
    //     )
    // }
}

#[pyfunction]
#[pyo3(signature = (*target), text_signature = "(*target: Retriever | int)")]
pub fn set(target: &Bound<'_, PyTuple>) -> PyResult<SetBuilder> {
    let (target, data_type) = idxes_from_tup(target)?;

    Ok(SetBuilder {
        target,
        target_data_type: data_type,
    })
}
