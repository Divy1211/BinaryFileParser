use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::{Bound, PyResult};
use pyo3::prelude::PyTupleMethods;
use pyo3::types::{PyAnyMethods, PyTuple};

use crate::errors::version_error::VersionError;
use crate::retrievers::retriever::{RetState, Retriever};
use crate::types::bfp_type::BfpType;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

// todo: add an if_supported
// todo: add grouped combinators


// todo: make an internal BfpError type for error data to not be lost when converting to a PyErr

pub fn idxes_from_tup(target: &Bound<PyTuple>) -> PyResult<(Vec<usize>, BfpType, String)> {
    if <Bound<PyTuple> as PyTupleMethods>::len(target) == 0 {
        return Err(PyValueError::new_err("Source/Target must contain at least one retriever"))
    }

    let Ok(fst) = target.get_item(0)?.extract::<Retriever>() else {
        return Err(PyValueError::new_err("Source/Target must begin with a retriever"))
    };

    let mut data_type = fst.data_type;
    let mut name = fst.name;
    let mut repeat = fst.repeat;
    let target = target.into_iter().map(|val| {
        if let Ok(ret) = val.extract::<Retriever>() {
            data_type = ret.data_type;
            name = ret.name;
            repeat = ret.repeat;
            return Ok(ret.idx);
        }
        if let Ok(idx) = val.extract::<usize>() {
            if repeat == 1 {
                data_type = data_type.get_contained_type(&name)?;
            }
            return Ok(idx);
        }
        Err(PyValueError::new_err("Only Retrievers or indexes may be specified in a path target. Use a single get[_len]() for arithmetic operations on int/list Retrievers"))
    }).collect::<PyResult<_>>()?;
    
    Ok((target, data_type, name))
}

pub fn get_rec(
    idxes: &[usize],
    retrievers: &[Retriever],
    data: &Vec<Option<ParseableType>>,
    ver: &Version,
) -> PyResult<(String, ParseableType)> {
    let idx = idxes[0];
    if idx > retrievers.len() {
        return Err(PyIndexError::new_err(
            "GetRec: Retriever index out of bounds"
        ));
    }
    let ret = &retrievers[idx];
    if idx >= data.len() {
        return Err(PyValueError::new_err(format!(
            "GetRec: '{}' has not been initialised yet", ret.name
        )));
    }
    let val = &data[idx];
    match val {
        None => {
            Err(VersionError::new_err(format!(
                "GetRec: '{}' is not supported in struct version {ver}", ret.name
            )))
        }
        Some(val) => {
            if idxes.len() == 1 {
                return Ok((ret.name.clone(), val.clone()));
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
) -> PyResult<(String, ParseableType)> {
    match val {
        ParseableType::Struct { val, struct_ } => {
            let inner = val.inner();

            get_rec(
                idxes,
                struct_.retrievers(),
                &inner.data,
                &inner.ver
            )
        },
        ParseableType::Array(ls) => {
            let idx = idxes[0];
            let inner = ls.inner();
            if idx > inner.data.len() {
                return Err(PyIndexError::new_err(format!(
                    "GetRec: List index out of bounds '{}'", name
                )));
            }
            if idxes.len() == 1 {
                return Ok((name.clone(), inner.data[idx].clone()));
            }
            get_from_parseable_type(&inner.data[idx], &idxes[1..], ver, name)
        },
        _ => {
            Err(VersionError::new_err(format!(
                "GetRec: Attempting sub-property/index access on non struct/list '{}'", name
            )))
        }
    }
}

pub fn set_rec(
    idxes: &[usize],
    retrievers: &[Retriever],
    data: &mut Vec<Option<ParseableType>>,
    repeats: &mut Vec<Option<isize>>,
    ver: &Version,
    val2: ParseableType,
) -> PyResult<()> {
    let idx = idxes[0];
    if idx > retrievers.len() {
        return Err(PyIndexError::new_err(
            "SetRec: Retriever index out of bounds"
        ));
    }
    let ret = &retrievers[idx];
    if idx >= data.len() {
        return Err(PyValueError::new_err(format!(
            "SetRec: '{}' has not been initialised yet", ret.name
        )));
    }
    let val = &data[idx];
    match val {
        None => {
            Err(VersionError::new_err(format!(
                "SetRec: '{}' is not supported in struct version {ver}", ret.name
            )))
        }
        Some(val) => {
            if idxes.len() == 1 {
                return set_data(retrievers, data, repeats, ver, idx, val2);
            }
            set_from_parseable_type(val, &idxes[1..], ver, &ret.name, val2)
        }
    }
}

fn set_from_parseable_type(
    val: &ParseableType,
    idxes: &[usize],
    ver: &Version,
    name: &String,
    val2: ParseableType,
) -> PyResult<()> {
    match val {
        ParseableType::Struct { val, struct_ } => {
            let mut inner = val.inner_mut();
            let (data, repeats, ver) = inner.split();

            set_rec(
                idxes,
                struct_.retrievers(),
                data,
                repeats,
                ver,
                val2,
            )
        },
        ParseableType::Array(ls) => {
            let idx = idxes[0];
            let mut inner = ls.inner_mut();
            if idx > inner.data.len() {
                return Err(PyIndexError::new_err(format!(
                    "SetRec: List index out of bounds '{}'", name
                )));
            }
            if idxes.len() == 1 {
                let Some(val2) = inner.data_type.try_cast(val2) else {
                    return Err(PyTypeError::new_err(format!(
                        "Set: Unable to set '{}' from value of incorrect type", name
                    )))
                };
                inner.data[idx] = val2;
                return Ok(())
            }
            set_from_parseable_type(&inner.data[idx], &idxes[1..], ver, name, val2)
        },
        _ => {
            Err(VersionError::new_err(format!(
                "SetRec: Attempting sub-property/index access on non struct/list '{}'", name
            )))
        }
    }
}

pub fn set_data(
    retrievers: &[Retriever],
    data: &mut Vec<Option<ParseableType>>,
    repeats: &mut Vec<Option<isize>>,
    ver: &Version,
    target: usize,
    val: ParseableType,
) -> PyResult<()> {
    let target_ret = &retrievers[target];

    if !target_ret.supported(ver) {
        return Err(VersionError::new_err(format!(
            "'{}' is not supported in struct version {ver}", target_ret.name
        )))
    }
    
    let state = target_ret.state(&repeats);
    
    if state == RetState::List || state == RetState::NoneList {
        if !val.is_ls_of(&target_ret.data_type) {
            return Err(PyTypeError::new_err(format!(
                "Set: Unable to set '{}' from value of incorrect type", target_ret.name
            )))
        }
        data[target] = Some(val);
        return Ok(())
    }
    
    let value = target_ret.data_type.try_cast(val);
    if value.is_none() {
        return Err(PyTypeError::new_err(format!(
            "Set: Unable to set '{}' from value of incorrect type", target_ret.name
        )))
    };
    data[target] = value;
    Ok(())
}