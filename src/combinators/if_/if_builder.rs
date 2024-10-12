use std::cmp::Ordering;

use pyo3::prelude::*;

use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::if_::if_cmp_from::IfCmpFrom;
use crate::combinators::if_::if_cmp_to::IfCmpTo;
use crate::retrievers::retriever::Retriever;
use crate::types::bfp_type::BfpType;
use crate::types::le::int::Int8;
use crate::types::parseable_type::ParseableType;

enum State {
    HasVal1,
    HasVal2,
    HasFixedVal2,
}

#[pyclass]
pub struct IfCmpBuilder {
    val1: usize,
    val1_type: BfpType,
    
    val2: Option<usize>,
    val2_const: Option<ParseableType>,

    ord: Option<Vec<Ordering>>,
    
    state: State,
    
    not: bool,
}

impl Default for IfCmpBuilder {
    fn default() -> Self {
        IfCmpBuilder {
            val1: 0,
            val1_type: BfpType::Int8(Int8),
            val2: None,
            val2_const: None,
            ord: None,
            state: State::HasVal1,
            not: false,
        }
    }
}

impl IfCmpBuilder {
    pub fn cmp_ret<'py>(&mut self, ret: &Bound<'py, Retriever>, ord: Vec<Ordering>) {
        let ret = ret.borrow();
        
        self.val2 = Some(ret.idx);
        self.ord = Some(ord);
        self.state = State::HasVal2;
        
    }
    pub fn cmp_fix<'py>(&mut self, val2: &Bound<PyAny>, ord: Vec<Ordering>) -> PyResult<()> {
        self.val2_const = Some(self.val1_type.to_parseable(&val2)?);
        self.ord = Some(ord);
        self.state = State::HasFixedVal2;
        
        Ok(())
    }
    
    pub fn cmp(&mut self, val2: Bound<PyAny>, ord: Vec<Ordering>) -> PyResult<()> {
        if let Ok(ret) = val2.downcast::<Retriever>() {
            self.cmp_ret(ret, ord);
        } else {
            self.cmp_fix(&val2, ord)?;
        }
        Ok(())
    }
}

#[pymethods]
impl IfCmpBuilder {
    fn then(&self, com: CombinatorType) -> PyResult<CombinatorType> {
        Ok(match self.state {
            State::HasVal1 => { todo!() }
            State::HasVal2 => {
                IfCmpFrom::new(
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
pub fn if_(val1: PyRef<Retriever>) -> IfCmpBuilder {
    IfCmpBuilder {
        val1: val1.idx,
        val1_type: val1.data_type.clone(),
        ..Default::default()
    }
}

#[pyfunction]
pub fn if_not(val1: PyRef<Retriever>) -> IfCmpBuilder {
    IfCmpBuilder {
        val1: val1.idx,
        val1_type: val1.data_type.clone(),
        not: true,
        ..Default::default()
    }
}