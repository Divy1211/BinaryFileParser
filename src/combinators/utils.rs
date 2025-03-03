use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::{Bound, PyResult};
use pyo3::prelude::PyTupleMethods;
use pyo3::types::{PyAnyMethods, PyTuple};

use crate::errors::version_error::VersionError;
use crate::retrievers::retriever::{RetState, Retriever};
use crate::types::bfp_type::BfpType;
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

pub fn idxes_from_tup(target: &Bound<PyTuple>) -> PyResult<(Vec<usize>, BfpType, String)> {
    if <Bound<PyTuple> as PyTupleMethods>::len(target) == 0 {
        return Err(PyValueError::new_err("Source/Target must contain at least one retriever"))
    }

    let Ok(fst) = target.get_item(0)?.extract::<Retriever>() else {
        return Err(PyValueError::new_err("Source/Target must begin with a retriever"))
    };

    let mut data_type = fst.data_type;
    let mut name = fst.name;
    let target = target.into_iter().map(|val| {
        val.extract::<Retriever>()
            .map(|ret| {
                data_type = ret.data_type;
                name = ret.name;
                Ok(ret.idx)
            })
            .unwrap_or_else(|_| val.extract::<usize>())
            .map_err(|_| {
                PyValueError::new_err("Only Retrievers or indexes may be specified in a path target. Use a single get[_len]() for arithmetic operations on int/list Retrievers")
            })
    }).collect::<PyResult<_>>()?;
    
    Ok((target, data_type, name))
}

pub fn get_rec(
    idxes: &[usize],
    retrievers: &Vec<Retriever>,
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
            let idx = idxes[0];
            let val = ls.ls.read().expect("GIL bound read");
            if idx > val.len() {
                return Err(PyIndexError::new_err(format!(
                    "GetRec: List index out of bounds '{}'", name
                )));
            }
            if idxes.len() == 1 {
                return Ok((name.clone(), val[idx].clone()));
            }
            get_from_parseable_type(&val[idx], &idxes[1..], ver, name)
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
    retrievers: &Vec<Retriever>,
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
            let mut sub_data = val.data.write().expect("GIL bound write");
            let mut sub_repeats = val.repeats.write().expect("GIL bound write");
            let sub_rets = struct_.retrievers.read().expect("GIL bound read");

            set_rec(
                idxes,
                &sub_rets,
                &mut sub_data,
                &mut sub_repeats,
                &val.ver,
                val2,
            )
        },
        ParseableType::Array(ls) => {
            let idx = idxes[0];
            let mut val3 = ls.ls.write().expect("GIL bound write");
            if idx > val3.len() {
                return Err(PyIndexError::new_err(format!(
                    "SetRec: List index out of bounds '{}'", name
                )));
            }
            if idxes.len() == 1 {
                if !val.is_ls_of(&ls.data_type) {
                    return Err(PyTypeError::new_err(format!(
                        "Set: Unable to set '{}' from value of incorrect type", name
                    )))
                }
                val3[idx] = val2.clone();
                return Ok(())
            }
            set_from_parseable_type(&val3[idx], &idxes[1..], ver, name, val2)
        },
        _ => {
            Err(VersionError::new_err(format!(
                "SetRec: Attempting sub-property/index access on non struct/list '{}'", name
            )))
        }
    }
}

pub fn set_data(
    retrievers: &Vec<Retriever>,
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