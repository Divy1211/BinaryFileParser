use pyo3::prelude::*;
use pyo3::types::PyTuple;

use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::set_repeat::set_repeat_from::SetRepeatFrom;
use crate::combinators::set_repeat::set_repeat_from_len::SetRepeatFromLen;
use crate::combinators::set_repeat::set_repeat_to::SetRepeatTo;
use crate::combinators::utils::idxes_from_tup;
use crate::retrievers::retriever::Retriever;

#[pyclass]
pub struct SetRepeatBuilder {
    target: usize
}

#[pymethods]
impl SetRepeatBuilder {

    #[pyo3(signature = (*source), text_signature = "(*source: Retriever | int)")]
    pub fn from_(&self, source: &Bound<PyTuple>) -> PyResult<CombinatorType> {
        let (source, _source_data_type, _source_name) = idxes_from_tup(source)?;
        
        Ok(SetRepeatFrom::new(self.target, source).into())
    }

    #[pyo3(signature = (*source), text_signature = "(*source: Retriever | int)")]
    pub fn from_len(&self, source: &Bound<PyTuple>) -> PyResult<CombinatorType> {
        let (source, _source_data_type, _source_name) = idxes_from_tup(source)?;

        Ok(SetRepeatFromLen::new(self.target, source).into())
    }
    
    pub fn to(&self, target: Bound<PyAny>) -> PyResult<CombinatorType> {
        Ok(SetRepeatTo::new(self.target, target.extract()?).into())
    }
}

#[pyfunction]
pub fn set_repeat(target: PyRef<Retriever>) -> SetRepeatBuilder {
    SetRepeatBuilder { target: target.idx }
}
