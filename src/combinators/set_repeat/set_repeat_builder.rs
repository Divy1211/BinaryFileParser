use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::get::Get;
use crate::combinators::set_repeat::set_repeat_by::SetRepeatBy;
use crate::combinators::set_repeat::set_repeat_from::SetRepeatFrom;
use crate::combinators::set_repeat::set_repeat_from_len::SetRepeatFromLen;
use crate::combinators::set_repeat::set_repeat_to::SetRepeatTo;
use crate::combinators::utils::idxes_from_tup;
use crate::retrievers::retriever::Retriever;

#[pyclass(module = "bfp_rs.combinators")]
pub struct SetRepeatBuilder {
    target: usize,
    target_name: String,
}

#[pymethods]
impl SetRepeatBuilder {
    pub fn by(&self, mut from: Get) -> PyResult<CombinatorType> {
        from.make_contiguous();
        Ok(SetRepeatBy::new(
            self.target,
            from,
        ).into())
    }
    
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
        let target = target.extract()?;
        if target < -2 {
            return Err(PyValueError::new_err(format!(
                "SetRepeatBy: Attempting to set repeat of '{}' to '{}', which is less than -2",
                self.target_name, target
            )));
        }
        Ok(SetRepeatTo::new(self.target, target).into())
    }
}

#[pyfunction]
pub fn set_repeat(target: PyRef<Retriever>) -> SetRepeatBuilder {
    SetRepeatBuilder { target: target.idx, target_name: target.name.clone() }
}
