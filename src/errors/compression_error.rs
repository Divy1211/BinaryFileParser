use pyo3::create_exception;
use crate::errors::parsing_error::ParsingError;

create_exception!(errors, CompressionError, ParsingError);