use pyo3::prelude::*;
use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::set_repeat::set_repeat_from::SetRepeatFrom;
use crate::combinators::set_repeat::set_repeat_to::SetRepeatTo;
use crate::retrievers::retriever::Retriever;

#[pyclass]
pub struct SetRepeatBuilder {
    of: usize
}

#[pymethods]
impl SetRepeatBuilder {
    pub fn from_(&self, from: Bound<Retriever>) -> CombinatorType {
        SetRepeatFrom::new(
            self.of,
            from.borrow().idx,
        ).into()
    }
    
    pub fn to(&self, val: Bound<PyAny>) -> PyResult<CombinatorType> {
        Ok(
            SetRepeatTo::new(
                self.of,
                val.extract()?,
            ).into()
        )
    }
}

#[pyfunction]
pub fn set_repeat(of: Bound<Retriever>) -> SetRepeatBuilder {
    let of = of.borrow().idx;
    SetRepeatBuilder { of }
}
