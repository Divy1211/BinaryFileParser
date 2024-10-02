use pyo3::pyclass;

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

    Bool,
    
    Str,
    
    Array,
    
    Bytes,
    
    Option,
    
    Struct,
}