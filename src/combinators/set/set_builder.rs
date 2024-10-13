use pyo3::prelude::*;
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::set::set_from::SetFrom;
use crate::combinators::set::set_from_len::SetFromLen;
use crate::combinators::set::set_to::SetTo;
use crate::retrievers::retriever::Retriever;
use crate::types::bfp_type::BfpType;

#[pyclass]
pub struct SetBuilder {
    target: usize,
    target_data_type: BfpType
}

#[pymethods]
impl SetBuilder {
    pub fn from_(&self, from: Bound<Retriever>) -> CombinatorType {
        SetFrom::new(
            self.target,
            from.borrow().idx,
        ).into()
    }

    pub fn from_len(&self, from: Bound<Retriever>) -> CombinatorType {
        SetFromLen::new(
            self.target,
            from.borrow().idx,
        ).into()
    }
    
    pub fn to(&self, val: &Bound<PyAny>) -> PyResult<CombinatorType> {
        Ok(
            SetTo::new(
                self.target,
                self.target_data_type.to_parseable(val)?, // todo: figure this out
            ).into()
        )
    }
}

#[pyfunction]
pub fn set(target: PyRef<Retriever>) -> SetBuilder {
    SetBuilder {
        target: target.idx,
        target_data_type: target.data_type.clone(),
    }
}
