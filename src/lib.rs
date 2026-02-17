#![allow(unexpected_cfgs)]
#![allow(unknown_lints, mismatched_lifetime_syntaxes)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::only_used_in_recursion)]

use pyo3::prelude::*;
use pyo3::py_run;

use crate::errors::compression_error::CompressionError;
use crate::errors::default_attribute_error::DefaultAttributeError;
use crate::errors::parsing_error::ParsingError;
use crate::errors::version_error::VersionError;
use crate::errors::mutability_error::MutabilityError;

use crate::combinators::set_repeat::set_repeat_builder::set_repeat;
use crate::combinators::r#if::if_builder::{if_, if_not, if_len, if_ver, if_not_key, if_key, if_else, break_};
use crate::combinators::set::set_builder::set;
use crate::combinators::set_key::set_key_builder::set_key;
use crate::combinators::get::{get_len, get, get_key, get_attr};

use crate::help::BorrowMutGuard;
use crate::help::set_mut;

use crate::retrievers::retriever::Retriever;
use crate::retrievers::retriever_combiner::RetrieverCombiner;
use crate::retrievers::retriever_ref::RetrieverRef;

use crate::types::base_struct::BaseStruct;
use crate::types::bfp_type::BfpType;
use crate::types::byte_stream::ByteStream;
use crate::types::context::ContextPtr;
use crate::types::diff_py::{ChangedPy, DeletedPy, DiffPy, InsertedPy, NestedDiffPy};
use crate::types::le::array::{Array, ArrayBuilder};
use crate::types::le::bool::{Bool128, Bool16, Bool32, Bool64, Bool8};
use crate::types::le::bytes::Bytes;
use crate::types::le::float::{Float32, Float64};
use crate::types::le::int::{UInt128, UInt16, UInt32, UInt64, UInt8, Int128, Int16, Int32, Int64, Int8};
use crate::types::le::encoding::Encoding;
use crate::types::le::nt_str::NtStr;
use crate::types::le::option::OptionBuilder;
use crate::types::le::size::Size;
use crate::types::le::stacked_array::{StackedArray, StackedArrayBuilder};
use crate::types::le::stacked_attr_array::{StackedAttrArray, StackedAttrArrayBuilder};
use crate::types::le::str::Str;
use crate::types::le::str_array::StrArray;
use crate::types::le::tail::Tail;
use crate::types::merge_py::{BasicPy, ConflictPy, NestedConflictPy};
use crate::types::ref_struct::RefStruct;
use crate::types::version::Version;

pub mod retrievers;
pub mod errors;
pub mod types;
pub mod macros;
pub mod combinators;
pub mod help;

fn le(py: Python, types: &Bound<PyModule>) -> PyResult<()> {
    let le = PyModule::new(types.py(), "bfp_rs.types.le")?;
    py_run!(py, le, "import sys; sys.modules['bfp_rs.types.le'] = le");
    types.add_submodule(&le)?;

    le.add("u8", BfpType::UInt8(UInt8))?;
    le.add("u16", BfpType::UInt16(UInt16))?;
    le.add("u32", BfpType::UInt32(UInt32))?;
    le.add("u64", BfpType::UInt64(UInt64))?;
    le.add("u128", BfpType::UInt128(UInt128))?;

    le.add("i8", BfpType::Int8(Int8))?;
    le.add("i16", BfpType::Int16(Int16))?;
    le.add("i32", BfpType::Int32(Int32))?;
    le.add("i64", BfpType::Int64(Int64))?;
    le.add("i128", BfpType::Int128(Int128))?;

    le.add("f32", BfpType::Float32(Float32))?;
    le.add("f64", BfpType::Float64(Float64))?;

    le.add("bool8", BfpType::Bool8(Bool8))?;
    le.add("bool16", BfpType::Bool16(Bool16))?;
    le.add("bool32", BfpType::Bool32(Bool32))?;
    le.add("bool64", BfpType::Bool64(Bool64))?;
    le.add("bool128", BfpType::Bool128(Bool128))?;

    le.add("str8", BfpType::Str(Str::len_size(Size::UInt8(UInt8))))?;
    le.add("str16", BfpType::Str(Str::len_size(Size::UInt16(UInt16))))?;
    le.add("str32", BfpType::Str(Str::len_size(Size::UInt32(UInt32))))?;
    le.add("str64", BfpType::Str(Str::len_size(Size::UInt64(UInt64))))?;
    le.add("str128", BfpType::Str(Str::len_size(Size::UInt128(UInt128))))?;

    le.add("c_str", BfpType::NTStr(NtStr::c_str()))?;
    le.add("nt_str8", BfpType::NTStr(NtStr::len_size(Size::UInt8(UInt8))))?;
    le.add("nt_str16", BfpType::NTStr(NtStr::len_size(Size::UInt16(UInt16))))?;
    le.add("nt_str32", BfpType::NTStr(NtStr::len_size(Size::UInt32(UInt32))))?;
    le.add("nt_str64", BfpType::NTStr(NtStr::len_size(Size::UInt64(UInt64))))?;
    le.add("nt_str128", BfpType::NTStr(NtStr::len_size(Size::UInt128(UInt128))))?;
    
    le.add("str_array8", BfpType::StrArray(StrArray::len_size(Size::UInt8(UInt8))))?;
    le.add("str_array16", BfpType::StrArray(StrArray::len_size(Size::UInt16(UInt16))))?;
    le.add("str_array32", BfpType::StrArray(StrArray::len_size(Size::UInt32(UInt32))))?;
    le.add("str_array64", BfpType::StrArray(StrArray::len_size(Size::UInt64(UInt64))))?;
    le.add("str_array128", BfpType::StrArray(StrArray::len_size(Size::UInt128(UInt128))))?;

    le.add("Option8", OptionBuilder::new(Size::UInt8(UInt8)))?;
    le.add("Option16", OptionBuilder::new(Size::UInt16(UInt16)))?;
    le.add("Option32", OptionBuilder::new(Size::UInt32(UInt32)))?;
    le.add("Option64", OptionBuilder::new(Size::UInt64(UInt64)))?;
    le.add("Option128", OptionBuilder::new(Size::UInt128(UInt128)))?;

    le.add("Array8", ArrayBuilder::new(Size::UInt8(UInt8)))?;
    le.add("Array16", ArrayBuilder::new(Size::UInt16(UInt16)))?;
    le.add("Array32", ArrayBuilder::new(Size::UInt32(UInt32)))?;
    le.add("Array64", ArrayBuilder::new(Size::UInt64(UInt64)))?;
    le.add("Array128", ArrayBuilder::new(Size::UInt128(UInt128)))?;

    le.add("StackedArray8", StackedArrayBuilder::new(Size::UInt8(UInt8)))?;
    le.add("StackedArray16", StackedArrayBuilder::new(Size::UInt16(UInt16)))?;
    le.add("StackedArray32", StackedArrayBuilder::new(Size::UInt32(UInt32)))?;
    le.add("StackedArray64", StackedArrayBuilder::new(Size::UInt64(UInt64)))?;
    le.add("StackedArray128", StackedArrayBuilder::new(Size::UInt128(UInt128)))?;

    le.add("StackedAttrArray8", StackedAttrArrayBuilder::new(Size::UInt8(UInt8)))?;
    le.add("StackedAttrArray16", StackedAttrArrayBuilder::new(Size::UInt16(UInt16)))?;
    le.add("StackedAttrArray32", StackedAttrArrayBuilder::new(Size::UInt32(UInt32)))?;
    le.add("StackedAttrArray64", StackedAttrArrayBuilder::new(Size::UInt64(UInt64)))?;
    le.add("StackedAttrArray128", StackedAttrArrayBuilder::new(Size::UInt128(UInt128)))?;
    
    le.add_class::<Bytes>()?;
    le.add_class::<Str>()?;
    le.add_class::<NtStr>()?;
    le.add_class::<Array>()?;
    le.add_class::<StackedArray>()?;
    le.add_class::<StackedAttrArray>()?;
    le.add_class::<Encoding>()?;
    le.add_class::<Tail>()?;

    le.add("void", BfpType::Bytes(Bytes { len: 0 }))?;
    
    Ok(())
}

fn types(py: Python, bfp: &Bound<PyModule>) -> PyResult<()> {
    let types = PyModule::new(bfp.py(), "bfp_rs.types")?;
    py_run!(py, types, "import sys; sys.modules['bfp_rs.types'] = types");
    bfp.add_submodule(&types)?;

    le(py, &types)?;

    Ok(())
}

fn combinators(py: Python, bfp: &Bound<PyModule>) -> PyResult<()> {
    let combinators = &PyModule::new(bfp.py(), "bfp_rs.combinators")?;
    py_run!(py, combinators, "import sys; sys.modules['bfp_rs.combinators'] = combinators");
    bfp.add_submodule(combinators)?;

    combinators.add_function(wrap_pyfunction!(set_repeat, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(if_, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(if_not, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(if_key, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(if_not_key, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(if_len, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(if_ver, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(if_else, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(break_, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(set, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(set_key, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(get, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(get_key, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(get_attr, combinators)?)?;
    combinators.add_function(wrap_pyfunction!(get_len, combinators)?)?;
    
    Ok(())
}

fn errors(py: Python, bfp: &Bound<PyModule>) -> PyResult<()> {
    let errors = PyModule::new(bfp.py(), "bfp_rs.errors")?;
    py_run!(py, errors, "import sys; sys.modules['bfp_rs.errors'] = errors");
    bfp.add_submodule(&errors)?;
    errors.add("ParsingError", py.get_type::<ParsingError>())?;
    errors.add("CompressionError", py.get_type::<CompressionError>())?;
    errors.add("DefaultValueError", py.get_type::<DefaultAttributeError>())?;
    errors.add("VersionError", py.get_type::<VersionError>())?;
    errors.add("MutabilityError", py.get_type::<MutabilityError>())?;

    Ok(())
}

fn diff(py: Python, bfp: &Bound<PyModule>) -> PyResult<()> {
    let diff = PyModule::new(bfp.py(), "bfp_rs.diff")?;
    py_run!(py, diff, "import sys; sys.modules['bfp_rs.diff'] = diff");
    bfp.add_submodule(&diff)?;

    diff.add_class::<DiffPy>()?;
    diff.add_class::<InsertedPy>()?;
    diff.add_class::<DeletedPy>()?;
    diff.add_class::<ChangedPy>()?;
    diff.add_class::<NestedDiffPy>()?;

    diff.add_class::<ConflictPy>()?;
    diff.add_class::<BasicPy>()?;
    diff.add_class::<NestedConflictPy>()?;
    
    Ok(())
}


#[pymodule]
#[pyo3(name = "bfp_rs")]
fn binary_file_parser(py: Python, bfp: &Bound<PyModule>) -> PyResult<()> {
    bfp.add_class::<ByteStream>()?;
    bfp.add_class::<BaseStruct>()?;
    bfp.add_class::<ContextPtr>()?;
    bfp.add_class::<Retriever>()?;
    bfp.add_class::<RetrieverRef>()?;
    bfp.add_class::<RetrieverCombiner>()?;
    bfp.add_class::<Version>()?;
    bfp.add_class::<RefStruct>()?;
    
    bfp.add_class::<BorrowMutGuard>()?;

    bfp.add_function(wrap_pyfunction!(set_mut, bfp)?)?;

    errors(py, bfp)?;
    types(py, bfp)?;
    diff(py, bfp)?;
    combinators(py, bfp)?;

    Ok(())
}