use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};

use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::get::Get;
use crate::combinators::set_key::set_key_by::SetKeyBy;
use crate::combinators::set_key::set_key_from::SetKeyFrom;
use crate::combinators::set_key::set_key_from_len::SetKeyFromLen;
use crate::combinators::utils::idxes_from_tup;

#[pyclass(module = "bfp_rs.combinators")]
pub struct SetKeyBuilder {
    key: String,
}

#[pymethods]
impl SetKeyBuilder {
    pub fn by(&self, mut from: Get) -> PyResult<CombinatorType> {
        from.make_contiguous();
        Ok(SetKeyBy::new(
            &self.key,
            from,
        ).into())
    }
    
    #[pyo3(signature = (*from), text_signature = "(*from: Retriever | int)")]
    pub fn from_(&self, from: &Bound<PyTuple>) -> PyResult<CombinatorType> {
        let (source, _source_data_type, _source_name) = idxes_from_tup(from)?;
        Ok(SetKeyFrom::new(
            &self.key,
            source,
        ).into())
    }

    #[pyo3(signature = (*from), text_signature = "(*from: Retriever | int)")]
    pub fn from_len(&self, from: &Bound<'_, PyTuple>) -> PyResult<CombinatorType> {
        let (source, _source_data_type, _source_name) = idxes_from_tup(from)?;
        Ok(SetKeyFromLen::new(
            &self.key,
            source,
        ).into())
    }
}

#[pyfunction]
pub fn set_key(key: &Bound<PyString>) -> PyResult<SetKeyBuilder> {
    Ok(SetKeyBuilder {
        key: key.to_string(),
    })
}
