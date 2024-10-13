use std::cmp::{Ordering, PartialEq};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::r#if::if_check::IfCheck;
use crate::combinators::r#if::if_cmp_from::IfCmpFrom;
use crate::combinators::r#if::if_cmp_len_from::IfCmpLenFrom;
use crate::combinators::r#if::if_cmp_len_to::IfCmpLenTo;
use crate::combinators::r#if::if_cmp_to::IfCmpTo;
use crate::retrievers::retriever::Retriever;
use crate::types::bfp_type::BfpType;
use crate::types::le::int::Int8;
use crate::types::parseable_type::ParseableType;

#[derive(Debug, PartialEq, Eq)]
enum State {
    HasTarget,
    HasSource,
    HasSourceConst,
}

#[pyclass]
pub struct IfBuilder {
    target: usize,
    target_data_type: BfpType,
    
    source: Option<usize>,
    source_const: Option<ParseableType>,

    ord: Option<Vec<Ordering>>,
    
    state: State,
    
    not: bool,
    len: bool,
}

impl Default for IfBuilder {
    fn default() -> Self {
        IfBuilder {
            target: 0,
            target_data_type: BfpType::Int8(Int8),
            source: None,
            source_const: None,
            ord: None,
            state: State::HasTarget,
            not: false,
            len: false,
        }
    }
}

impl IfBuilder {
    pub fn cmp_ret<'py>(&mut self, ret: &Bound<'py, Retriever>, ord: Vec<Ordering>) {
        let ret = ret.borrow();
        
        self.source = Some(ret.idx);
        self.ord = Some(ord);
        self.state = State::HasSource;
        
    }
    pub fn cmp_fix<'py>(&mut self, source: &Bound<PyAny>, ord: Vec<Ordering>) -> PyResult<()> {
        if self.len {
            let val2 = source.extract::<isize>()?;
            if val2 < 0 {
                return Err(PyValueError::new_err(
                    "Using a negative value in a length comparison is a bug"
                ))
            }
            self.source = Some(val2 as usize);
        } else {
            self.source_const = Some(self.target_data_type.to_parseable(&source)?)
        };
        self.ord = Some(ord);
        self.state = State::HasSourceConst;
        
        Ok(())
    }
    
    pub fn cmp(&mut self, source: Bound<PyAny>, ord: Vec<Ordering>) -> PyResult<()> {
        if self.state != State::HasTarget {
            return Err(PyTypeError::new_err(
                "Cannot chain comparisons, use a .then() with a nested if_"
            ))
        }
        
        if let Ok(ret) = source.downcast::<Retriever>() {
            self.cmp_ret(ret, ord);
        } else {
            self.cmp_fix(&source, ord)?;
        }
        Ok(())
    }
}

#[pymethods]
impl IfBuilder {
    fn then(&self, com: CombinatorType) -> PyResult<CombinatorType> {
        Ok(match self.state {
            State::HasTarget => {
                IfCheck::new(
                    self.target,
                    com
                ).into()
            }
            State::HasSource if self.len => {
                IfCmpLenFrom::new(
                    self.target,
                    self.source.expect("infallible"),
                    self.ord.clone().expect("infallible"),
                    com,
                ).into()
            }
            State::HasSource => {
                IfCmpFrom::new(
                    self.target,
                    self.source.expect("infallible"),
                    self.ord.clone().expect("infallible"),
                    com,
                ).into()
            }
            State::HasSourceConst if self.len => {
                IfCmpLenTo::new(
                    self.target,
                    self.source.expect("infallible"),
                    self.ord.clone().expect("infallible"),
                    com,
                ).into()
            }
            State::HasSourceConst => {
                IfCmpTo::new(
                    self.target,
                    self.source_const.clone().expect("infallible"),
                    self.ord.clone().expect("infallible"),
                    com,
                ).into()
            }
        })
    }
    
    fn eq<'py>(slf: Bound<'py, Self>, source: Bound<PyAny>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(source, vec![Ordering::Less, Ordering::Greater])?;
        } else {
            this.cmp(source, vec![Ordering::Equal])?;
        }
        Ok(slf)
    }

    fn neq<'py>(slf: Bound<'py, Self>, source: Bound<PyAny>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(source, vec![Ordering::Equal])?;
        } else {
            this.cmp(source, vec![Ordering::Less, Ordering::Greater])?;
        }
        Ok(slf)
    }

    fn gt<'py>(slf: Bound<'py, Self>, source: Bound<PyAny>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(source, vec![Ordering::Less, Ordering::Equal])?;
        } else {
            this.cmp(source, vec![Ordering::Greater])?;
        }
        Ok(slf)
    }

    fn geq<'py>(slf: Bound<'py, Self>, source: Bound<PyAny>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(source, vec![Ordering::Less])?;
        } else {
            this.cmp(source, vec![Ordering::Greater, Ordering::Equal])?;
        }
        Ok(slf)
    }

    fn lt<'py>(slf: Bound<'py, Self>, source: Bound<PyAny>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(source, vec![Ordering::Greater, Ordering::Equal])?;
        } else {
            this.cmp(source, vec![Ordering::Less])?;
        }
        Ok(slf)
    }
    
    fn leq<'py>(slf: Bound<'py, Self>, source: Bound<PyAny>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(source, vec![Ordering::Greater])?;
        } else {
            this.cmp(source, vec![Ordering::Less, Ordering::Equal])?;
        }
        Ok(slf)
    }
}

#[pyfunction]
pub fn if_(source: PyRef<Retriever>) -> IfBuilder {
    IfBuilder {
        target: source.idx,
        target_data_type: source.data_type.clone(),
        ..Default::default()
    }
}

#[pyfunction]
pub fn if_not(source: PyRef<Retriever>) -> IfBuilder {
    IfBuilder {
        target: source.idx,
        target_data_type: source.data_type.clone(),
        not: true,
        ..Default::default()
    }
}

#[pyfunction]
pub fn if_len(source: PyRef<Retriever>) -> IfBuilder {
    IfBuilder {
        target: source.idx,
        target_data_type: source.data_type.clone(),
        len: true,
        ..Default::default()
    }
}