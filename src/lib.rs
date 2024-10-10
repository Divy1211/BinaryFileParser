use pyo3::prelude::*;
use pyo3::py_run;

use crate::errors::compression_error::CompressionError;
use crate::errors::default_attribute_error::DefaultAttributeError;
use crate::errors::parsing_error::ParsingError;
use crate::errors::version_error::VersionError;

pub mod retrievers;
pub mod errors;
pub mod types;
pub mod macros;

fn le(py: Python, types: &Bound<PyModule>) -> PyResult<()> {
    let le = PyModule::new_bound(types.py(), "bfp_rs.types.le")?;
    py_run!(py, le, "import sys; sys.modules['bfp_rs.types.le'] = le");
    types.add_submodule(&le)?;

    le.add_class::<types::le::int::UInt8>()?;
    le.add_class::<types::le::int::UInt16>()?;
    le.add_class::<types::le::int::UInt32>()?;
    le.add_class::<types::le::int::UInt64>()?;
    le.add_class::<types::le::int::UInt128>()?;
    
    le.add_class::<types::le::int::Int8>()?;
    le.add_class::<types::le::int::Int16>()?;
    le.add_class::<types::le::int::Int32>()?;
    le.add_class::<types::le::int::Int64>()?;
    le.add_class::<types::le::int::Int128>()?;
    
    le.add_class::<types::le::float::Float32>()?;
    le.add_class::<types::le::float::Float64>()?;

    le.add_class::<types::le::bool::Bool8>()?;
    le.add_class::<types::le::bool::Bool16>()?;
    le.add_class::<types::le::bool::Bool32>()?;
    le.add_class::<types::le::bool::Bool64>()?;
    le.add_class::<types::le::bool::Bool128>()?;
    
    Ok(())
}

fn types(py: Python, bfp: &Bound<PyModule>) -> PyResult<()> {
    let types = PyModule::new_bound(bfp.py(), "bfp_rs.types")?;
    py_run!(py, types, "import sys; sys.modules['bfp_rs.types'] = types");
    bfp.add_submodule(&types)?;
    types.add_class::<types::version::Version>()?;
    types.add_class::<types::bfp_type::BfpType>()?;

    le(py, &types)?;

    Ok(())
}

fn errors(py: Python, bfp: &Bound<PyModule>) -> PyResult<()> {
    let errors = PyModule::new_bound(bfp.py(), "bfp_rs.errors")?;
    py_run!(py, errors, "import sys; sys.modules['bfp_rs.errors'] = errors");
    bfp.add_submodule(&errors)?;
    errors.add("ParsingError", py.get_type_bound::<ParsingError>())?;
    errors.add("CompressionError", py.get_type_bound::<CompressionError>())?;
    errors.add("DefaultValueError", py.get_type_bound::<DefaultAttributeError>())?;
    errors.add("VersionError", py.get_type_bound::<VersionError>())?;

    Ok(())
}

#[pymodule]
#[pyo3(name = "bfp_rs")]
fn binary_file_parser(py: Python, bfp: &Bound<PyModule>) -> PyResult<()> {
    bfp.add_class::<types::byte_stream::ByteStream>()?;
    bfp.add_class::<types::base_struct::BaseStruct>()?;
    bfp.add_class::<types::r#struct::Struct>()?;
    bfp.add_class::<retrievers::retriever::Retriever>()?;

    errors(py, bfp)?;
    types(py, bfp)?;

    Ok(())
}