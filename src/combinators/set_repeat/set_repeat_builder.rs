use pyo3::prelude::*;
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::set_repeat::set_repeat_from::SetRepeatFrom;
use crate::combinators::set_repeat::set_repeat_to::SetRepeatTo;
use crate::retrievers::retriever::Retriever;

#[pyclass]
pub struct SetRepeatBuilder {
    target: usize
}

#[pymethods]
impl SetRepeatBuilder {
    pub fn from_(&self, target: Bound<Retriever>) -> CombinatorType {
        SetRepeatFrom::new(
            self.target,
            target.borrow().idx,
        ).into()
    }
    
    pub fn to(&self, target: Bound<PyAny>) -> PyResult<CombinatorType> {
        Ok(
            SetRepeatTo::new(
                self.target,
                target.extract()?,
            ).into()
        )
    }
}

#[pyfunction]
pub fn set_repeat(target: PyRef<Retriever>) -> SetRepeatBuilder {
    SetRepeatBuilder { target: target.idx }
}
