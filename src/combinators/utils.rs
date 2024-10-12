use pyo3::exceptions::PyValueError;
use pyo3::PyResult;
use crate::errors::version_error::VersionError;
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[cfg_attr(feature = "inline_always", inline(always))]
pub fn check_initialized(
    idx: usize,
    retrievers: &Vec<Retriever>,
    data: &Vec<Option<ParseableType>>
) -> PyResult<()> {
    if idx >= data.len() {
        return Err(PyValueError::new_err(format!(
            "SetRepeat: '{}' has not been initialised yet", retrievers[idx].name
        )))
    }
    
    Ok(())
}

#[cfg_attr(feature = "inline_always", inline(always))]
pub fn get<'a>(
    idx: usize,
    retrievers: &Vec<Retriever>,
    data: &'a Vec<Option<ParseableType>>,
    ver: &Version,
) -> PyResult<&'a ParseableType> {
    let Some(repeat) = &data[idx] else {
        return Err(VersionError::new_err(format!(
            "SetRepeat: '{}' is not supported in struct version {ver}", retrievers[idx].name
        )))
    };
    
    Ok(repeat)
}