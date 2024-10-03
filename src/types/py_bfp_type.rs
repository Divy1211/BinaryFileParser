use pyo3::{pyclass, PyObject, PyResult, Python};

use crate::types::bfp_type::BfpType;
use crate::types::le::bool::{Bool128, Bool16, Bool32, Bool64, Bool8};
use crate::types::le::float::{Float32, Float64};
use crate::types::le::int::{Int128, Int16, Int32, Int64, Int8, UInt128, UInt16, UInt32, UInt64, UInt8};

#[pyclass(module = "bfp_rs", eq, eq_int)]
#[derive(PartialEq, Eq, Clone)]
pub enum PyBfpType {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,

    Int8,
    Int16,
    Int32,
    Int64,
    Int128,

    Float32,
    Float64,

    Bool8,
    Bool16,
    Bool32,
    Bool64,
    Bool128,

    Str,
    
    Array,

    Bytes,

    Option,

    Struct,
}

pub fn to_bfp_type(
    py: Python,
    data_type: PyObject,
) -> PyResult<BfpType> {
    let py_bfp_type = data_type
        .getattr(py, "to_py_bfp_type")?
        .call_bound(py, (), None)?
        .extract::<PyBfpType>(py)?;

    Ok(match py_bfp_type {
        PyBfpType::UInt8 => data_type.extract::<UInt8>(py)?.into(),
        PyBfpType::UInt16 => data_type.extract::<UInt16>(py)?.into(),
        PyBfpType::UInt32 => data_type.extract::<UInt32>(py)?.into(),
        PyBfpType::UInt64 => data_type.extract::<UInt64>(py)?.into(),
        PyBfpType::UInt128 => data_type.extract::<UInt128>(py)?.into(),

        PyBfpType::Int8 => data_type.extract::<Int8>(py)?.into(),
        PyBfpType::Int16 => data_type.extract::<Int16>(py)?.into(),
        PyBfpType::Int32 => data_type.extract::<Int32>(py)?.into(),
        PyBfpType::Int64 => data_type.extract::<Int64>(py)?.into(),
        PyBfpType::Int128 => data_type.extract::<Int128>(py)?.into(),

        PyBfpType::Float32 => data_type.extract::<Float32>(py)?.into(),
        PyBfpType::Float64 => data_type.extract::<Float64>(py)?.into(),

        PyBfpType::Bool8 => data_type.extract::<Bool8>(py)?.into(),
        PyBfpType::Bool16 => data_type.extract::<Bool16>(py)?.into(),
        PyBfpType::Bool32 => data_type.extract::<Bool32>(py)?.into(),
        PyBfpType::Bool64 => data_type.extract::<Bool64>(py)?.into(),
        PyBfpType::Bool128 => data_type.extract::<Bool128>(py)?.into(),

        _ => { todo!() }
    })
}