use std::cmp::{Ordering, PartialEq};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::if_::if_check::IfCheck;
use crate::combinators::if_::if_cmp_from::IfCmpFrom;
use crate::combinators::if_::if_cmp_len_from::IfCmpLenFrom;
use crate::combinators::if_::if_cmp_len_to::IfCmpLenTo;
use crate::combinators::if_::if_cmp_to::IfCmpTo;
use crate::retrievers::retriever::Retriever;
use crate::types::bfp_type::BfpType;
use crate::types::le::int::Int8;
use crate::types::parseable_type::ParseableType;

#[derive(Debug, PartialEq, Eq)]
enum State {
    HasVal1,
    HasVal2,
    HasFixedVal2,
}

#[pyclass]
pub struct IfBuilder {
    val1: usize,
    val1_type: BfpType,
    
    val2: Option<usize>,
    val2_const: Option<ParseableType>,

    ord: Option<Vec<Ordering>>,
    
    state: State,
    
    not: bool,
    len: bool,
}

impl Default for IfBuilder {
    fn default() -> Self {
        IfBuilder {
            val1: 0,
            val1_type: BfpType::Int8(Int8),
            val2: None,
            val2_const: None,
            ord: None,
            state: State::HasVal1,
            not: false,
            len: false,
        }
    }
}

impl IfBuilder {
    pub fn cmp_ret<'py>(&mut self, ret: &Bound<'py, Retriever>, ord: Vec<Ordering>) {
        let ret = ret.borrow();
        
        self.val2 = Some(ret.idx);
        self.ord = Some(ord);
        self.state = State::HasVal2;
        
    }
    pub fn cmp_fix<'py>(&mut self, val2: &Bound<PyAny>, ord: Vec<Ordering>) -> PyResult<()> {
        if self.len {
            let val2 = val2.extract::<isize>()?;
            if val2 < 0 {
                return Err(PyValueError::new_err(
                    "Using a negative value in a length comparison is a bug"
                ))
            }
            self.val2 = Some(val2 as usize);
        } else {
            self.val2_const = Some(self.val1_type.to_parseable(&val2)?)
        };
        self.ord = Some(ord);
        self.state = State::HasFixedVal2;
        
        Ok(())
    }
    
    pub fn cmp(&mut self, val2: Bound<PyAny>, ord: Vec<Ordering>) -> PyResult<()> {
        if self.state != State::HasVal1 {
            return Err(PyTypeError::new_err(
                "Cannot chain comparisons, use a .then() with a nested if_"
            ))
        }
        
        if let Ok(ret) = val2.downcast::<Retriever>() {
            self.cmp_ret(ret, ord);
        } else {
            self.cmp_fix(&val2, ord)?;
        }
        Ok(())
    }
}

#[pymethods]
impl IfBuilder {
    fn then(&self, com: CombinatorType) -> PyResult<CombinatorType> {
        Ok(match self.state {
            State::HasVal1 => {
                IfCheck::new(
                    self.val1,
                    com
                ).into()
            }
            State::HasVal2 if self.len => {
                IfCmpLenFrom::new(
                    self.val1,
                    self.val2.expect("infallible"),
                    self.ord.clone().expect("infallible"),
                    com,
                ).into()
            }
            State::HasVal2 => {
                IfCmpFrom::new(
                    self.val1,
                    self.val2.expect("infallible"),
                    self.ord.clone().expect("infallible"),
                    com,
                ).into()
            }
            State::HasFixedVal2 if self.len => {
                IfCmpLenTo::new(
                    self.val1,
                    self.val2.expect("infallible"),
                    self.ord.clone().expect("infallible"),
                    com,
                ).into()
            }
            State::HasFixedVal2 => {
                IfCmpTo::new(
                    self.val1,
                    self.val2_const.clone().expect("infallible"),
                    self.ord.clone().expect("infallible"),
                    com,
                ).into()
            }
        })
    }
    
    fn eq<'py>(slf: Bound<'py, Self>, val2: Bound<PyAny>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(val2, vec![Ordering::Less, Ordering::Greater])?;
        } else {
            this.cmp(val2, vec![Ordering::Equal])?;
        }
        Ok(slf)
    }

    fn neq<'py>(slf: Bound<'py, Self>, val2: Bound<PyAny>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(val2, vec![Ordering::Equal])?;
        } else {
            this.cmp(val2, vec![Ordering::Less, Ordering::Greater])?;
        }
        Ok(slf)
    }

    fn gt<'py>(slf: Bound<'py, Self>, val2: Bound<PyAny>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(val2, vec![Ordering::Less, Ordering::Equal])?;
        } else {
            this.cmp(val2, vec![Ordering::Greater])?;
        }
        Ok(slf)
    }

    fn geq<'py>(slf: Bound<'py, Self>, val2: Bound<PyAny>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(val2, vec![Ordering::Less])?;
        } else {
            this.cmp(val2, vec![Ordering::Greater, Ordering::Equal])?;
        }
        Ok(slf)
    }

    fn lt<'py>(slf: Bound<'py, Self>, val2: Bound<PyAny>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(val2, vec![Ordering::Greater, Ordering::Equal])?;
        } else {
            this.cmp(val2, vec![Ordering::Less])?;
        }
        Ok(slf)
    }
    
    fn leq<'py>(slf: Bound<'py, Self>, val2: Bound<PyAny>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(val2, vec![Ordering::Greater])?;
        } else {
            this.cmp(val2, vec![Ordering::Less, Ordering::Equal])?;
        }
        Ok(slf)
    }
}

#[pyfunction]
pub fn if_(val1: PyRef<Retriever>) -> IfBuilder {
    IfBuilder {
        val1: val1.idx,
        val1_type: val1.data_type.clone(),
        ..Default::default()
    }
}

#[pyfunction]
pub fn if_not(val1: PyRef<Retriever>) -> IfBuilder {
    IfBuilder {
        val1: val1.idx,
        val1_type: val1.data_type.clone(),
        not: true,
        ..Default::default()
    }
}

#[pyfunction]
pub fn if_len(val1: PyRef<Retriever>) -> IfBuilder {
    IfBuilder {
        val1: val1.idx,
        val1_type: val1.data_type.clone(),
        len: true,
        ..Default::default()
    }
}