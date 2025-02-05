use std::process::id;
use pyo3::exceptions::{PyIndexError, PyValueError};
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
            "Combinator: '{}' has not been initialised yet", retrievers[idx].name
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
            "Combinator: '{}' is not supported in struct version {ver}", retrievers[idx].name
        )))
    };
    
    Ok(repeat)
}

pub fn get_rec(
    idxes: &[usize],
    retrievers: &Vec<Retriever>,
    data: &Vec<Option<ParseableType>>,
    ver: &Version,
) -> PyResult<ParseableType> {
    if idxes.len() == 0 {
        panic!("BFP Combinator Recursive get Internal Error.")
    }
    let idx = idxes[0];
    if idx > retrievers.len() {
        return Err(PyIndexError::new_err(
            "Combinator: Retriever index out of bounds"
        ));
    }
    let ret = &retrievers[idx];
    if idx >= data.len() {
        return Err(PyValueError::new_err(format!(
            "Combinator: '{}' has not been initialised yet", ret.name
        )));
    }
    let val = &data[idx];
    match val {
        None => {
            Err(VersionError::new_err(format!(
                "Combinator: '{}' is not supported in struct version {ver}", ret.name
            )))
        }
        Some(val) => {
            if idxes.len() == 1 {
                return Ok(val.clone());
            }
            get_from_parseable_type(val, &idxes[1..], ver, &ret.name)
        }
    }
}

fn get_from_parseable_type(
    val: &ParseableType,
    idxes: &[usize],
    ver: &Version,
    name: &String,
) -> PyResult<ParseableType> {
    match val {
        ParseableType::Struct { val, struct_ } => {
            let sub_data = val.data.read().expect("GIL bound read");
            let sub_rets = struct_.retrievers.read().expect("GIL bound read");

            get_rec(
                idxes,
                &sub_rets,
                &sub_data,
                &val.ver
            )
        },
        ParseableType::Array(ls) => {
            if idxes.len() == 1 {
                return Ok(val.clone());
            }
            let idx = idxes[0];
            let val = ls.ls.read().expect("GIL bound read");
            if idx > val.len() {
                return Err(PyIndexError::new_err(format!(
                    "Combinator: List index out of bounds '{}'", name
                )));
            }
            get_from_parseable_type(&val[idx], &idxes[1..], ver, name)
        },
        _ => {
            Err(VersionError::new_err(format!(
                "Combinator: Attempting sub-property/index access on non struct/list '{}'", name
            )))
        }
    }
}