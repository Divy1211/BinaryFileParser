use std::collections::VecDeque;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};

use crate::combinators::utils::{get_rec, idxes_from_tup};
use crate::retrievers::retriever::Retriever;
use crate::types::base_struct::BaseStruct;
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::struct_builder::StructBuilder;
use crate::types::version::Version;

#[derive(Debug, Clone)]
pub enum Item {
    Int(i128),
    Ref(Vec<usize>),
    RefLen(Vec<usize>),
    CtxKey(String),
    Attr(String),
    
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    BitAnd,
    BitOr,
    BitXor,

    Neg,
    BitNeg,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Get {
    /// reverse polish notation
    pub rpn: VecDeque<Item>,
}

impl Get {
    fn op(&mut self, other: Bound<PyAny>, op: Item, rev: bool) -> PyResult<()> {
        if let Ok(other) = other.extract::<Self>() {
            self.rpn.reserve(other.rpn.len());
            if rev {
                for item in other.rpn.into_iter().rev() {
                    self.rpn.push_front(item);
                }
            } else {
                for item in other.rpn {
                    self.rpn.push_back(item);
                }
            }
            self.rpn.push_back(op);
        } else if let Ok(num) = other.extract::<i128>() {
            if rev {
                self.rpn.push_front(Item::Int(num));
            } else {
                self.rpn.push_back(Item::Int(num));
            }
            self.rpn.push_back(op);
        } else {
            return Err(PyValueError::new_err("Cannot operate on values other than refs and ints"));
        }
        Ok(())
    }

    pub fn new(target: Item) -> Self {
        Self {
            rpn: VecDeque::from([target]),
        }
    }
    
    pub fn eval(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        _repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &Context,
    ) -> PyResult<i128> {
        let mut stack = Vec::new();

        for item in &self.rpn {
            match item {
                Item::Int(num) => {
                    stack.push(*num)
                },
                Item::Ref(source) => {
                    let (name, val) = get_rec(source, retrievers, data, ver)?;
                    let Some(val) = val.try_to_int() else {
                        return Err(PyValueError::new_err(format!("'{}' cannot be interpreted as an int", name)))
                    };
                    stack.push(val);
                },
                Item::RefLen(source) => {
                    let (name, val) = get_rec(source, retrievers, data, ver)?;
                    let Some(val) = val.try_len() else {
                        return Err(PyValueError::new_err(format!("'{}' cannot be interpreted as a list", name)))
                    };
                    stack.push(val as i128);
                },
                Item::CtxKey(key) => {
                    let val = ctx.get(key)?;
                    let Some(val) = val.try_to_int() else {
                        return Err(PyValueError::new_err(format!("Context key '{}' cannot be interpreted as an int", key)))
                    };
                    stack.push(val);
                }
                Item::Attr(attr) => {
                    return Err(PyValueError::new_err(format!("get_attr '{}' is only allowed in RefStructs", attr)));
                }
                Item::Add => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 + op2);
                }
                Item::Sub => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 - op2);
                }
                Item::Mul => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 * op2);
                }
                Item::Div => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 / op2);
                }
                Item::Mod => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1.rem_euclid(op2));
                }
                Item::BitAnd => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 & op2);
                }
                Item::BitOr => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 | op2);
                }
                Item::BitXor => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 ^ op2);
                }
                Item::Neg => {
                    let op1 = stack.pop().expect("By construction");
                    stack.push(-op1);
                }
                Item::BitNeg => {
                    let op1 = stack.pop().expect("By construction");
                    stack.push(!op1);
                }
            }
        }

        Ok(stack.pop().expect("By construction"))
    }

    pub fn eval_ref(
        &self,
        ref_struct: &Bound<PyAny>,
        struct_: &Bound<PyAny>,
    ) -> PyResult<i128> {
        let mut stack = Vec::new();

        for item in &self.rpn {
            match item {
                Item::Int(num) => {
                    stack.push(*num)
                },
                Item::Ref(source) => {
                    let borrow = struct_.cast::<BaseStruct>()?.borrow();
                    let struct_ = StructBuilder::get_struct(&struct_.get_type())?;
                    let inner = borrow.inner();

                    let (name, val) = get_rec(source, struct_.retrievers(), &inner.data, &inner.ver)?;
                    let Some(val) = val.try_to_int() else {
                        return Err(PyValueError::new_err(format!("'{}' cannot be interpreted as an int", name)))
                    };
                    stack.push(val);
                },
                Item::RefLen(source) => {
                    let borrow = struct_.cast::<BaseStruct>()?.borrow();
                    let struct_ = StructBuilder::get_struct(&struct_.get_type())?;
                    let inner = borrow.inner();

                    let (name, val) = get_rec(source, struct_.retrievers(), &inner.data, &inner.ver)?;
                    let Some(val) = val.try_len() else {
                        return Err(PyValueError::new_err(format!("'{}' cannot be interpreted as a list", name)))
                    };
                    stack.push(val as i128);
                },
                Item::CtxKey(key) => {
                    return Err(PyValueError::new_err(format!("get_key '{}' is only allowed in combinators", key)));
                }
                Item::Attr(attr) => {
                    let val = ref_struct.getattr(attr.as_str())?;
                    let Ok(val) = val.extract::<i128>() else {
                        return Err(PyValueError::new_err(format!("Attribute '{}' cannot be interpreted as an int", attr)))
                    };
                    stack.push(val);
                }
                Item::Add => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 + op2);
                }
                Item::Sub => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 - op2);
                }
                Item::Mul => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 * op2);
                }
                Item::Div => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 / op2);
                }
                Item::Mod => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1.rem_euclid(op2));
                }
                Item::BitAnd => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 & op2);
                }
                Item::BitOr => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 | op2);
                }
                Item::BitXor => {
                    let op2 = stack.pop().expect("By construction");
                    let op1 = stack.pop().expect("By construction");
                    stack.push(op1 ^ op2);
                }
                Item::Neg => {
                    let op1 = stack.pop().expect("By construction");
                    stack.push(-op1);
                }
                Item::BitNeg => {
                    let op1 = stack.pop().expect("By construction");
                    stack.push(!op1);
                }
            }
        }

        Ok(stack.pop().expect("By construction"))
    }
    
    pub fn make_contiguous(&mut self) {
        self.rpn.make_contiguous();
    }
}

// this is all boilerplate, refer to the op fn
#[pymethods]
impl Get {
    pub fn __add__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::Add, false)?;
        Ok(slf)
    }
    pub fn __radd__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::Add, true)?;
        Ok(slf)
    }
    pub fn __sub__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::Sub, false)?;
        Ok(slf)
    }
    pub fn __rsub__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::Sub, true)?;
        Ok(slf)
    }
    pub fn __mul__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::Mul, false)?;
        Ok(slf)
    }
    pub fn __rmul__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::Mul, true)?;
        Ok(slf)
    }
    pub fn __div__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::Div, false)?;
        Ok(slf)
    }
    pub fn __rdiv__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::Div, true)?;
        Ok(slf)
    }
    pub fn __mod__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::Mod, false)?;
        Ok(slf)
    }
    pub fn __rmod__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::Mod, true)?;
        Ok(slf)
    }
    
    pub fn __and__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::BitAnd, false)?;
        Ok(slf)
    }
    pub fn __rand__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::BitAnd, true)?;
        Ok(slf)
    }
    pub fn __or__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::BitOr, false)?;
        Ok(slf)
    }
    pub fn __ror__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::BitOr, true)?;
        Ok(slf)
    }
    pub fn __xor__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::BitXor, false)?;
        Ok(slf)
    }
    pub fn __rxor__<'py>(mut slf: PyRefMut<'py, Self>, other: Bound<PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        slf.op(other, Item::BitXor, true)?;
        Ok(slf)
    }

    pub fn __neg__(mut slf: PyRefMut<'_, Self>) -> PyResult<PyRefMut<'_, Self>> {
        slf.rpn.push_back(Item::Neg);
        Ok(slf)
    }
    pub fn __invert__(mut slf: PyRefMut<'_, Self>) -> PyResult<PyRefMut<'_, Self>> {
        slf.rpn.push_back(Item::BitNeg);
        Ok(slf)
    }
}

#[pyfunction]
#[pyo3(signature = (*source), text_signature = "(*source: Retriever | int)")]
pub fn get(source: &Bound<PyTuple>) -> PyResult<Get> {
    let (source, _source_data_type, _source_name) = idxes_from_tup(source)?;
    Ok(Get::new(Item::Ref(source)))
}

#[pyfunction]
pub fn get_key(key: &Bound<PyString>) -> Get {
    Get::new(Item::CtxKey(key.to_string()))
}

#[pyfunction]
pub fn get_attr(attr: &Bound<PyString>) -> Get {
    Get::new(Item::Attr(attr.to_string()))
}

#[pyfunction]
#[pyo3(signature = (*source), text_signature = "(*source: Retriever | int)")]
pub fn get_len(source: &Bound<PyTuple>) -> PyResult<Get> {
    let (source, _source_data_type, _source_name) = idxes_from_tup(source)?;
    Ok(Get::new(Item::RefLen(source)))
}