use std::cmp::{Ordering, PartialEq};

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyTuple;

use crate::combinators::combinator_type::CombinatorType;
use crate::combinators::get::Get;
use crate::combinators::r#if::if_check::IfCheck;
use crate::combinators::r#if::if_cmp_by::IfCmpBy;
use crate::combinators::r#if::if_cmp_from::IfCmpFrom;
use crate::combinators::r#if::if_cmp_len_by::IfCmpLenBy;
use crate::combinators::r#if::if_cmp_len_from::IfCmpLenFrom;
use crate::combinators::r#if::if_cmp_len_to::IfCmpLenTo;
use crate::combinators::r#if::if_cmp_to::IfCmpTo;
use crate::combinators::r#if::if_is_none::IfIsNone;
use crate::combinators::r#if::if_ver::IfVer;
use crate::combinators::utils::idxes_from_tup;
use crate::retrievers::retriever::Retriever;
use crate::types::bfp_type::BfpType;
use crate::types::le::int::Int8;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[derive(Debug, PartialEq, Eq)]
enum State {
    VerCheck,
    HasTarget,
    HasSource,
    HasSourceConst,
    HasSourceGet,
}

#[pyclass(module = "bfp_rs.combinators")]
pub struct IfBuilder {
    target: Vec<usize>,
    target_data_type: BfpType,

    min_ver: Option<Version>,
    max_ver: Option<Version>,
    
    source: Option<Vec<usize>>,
    source_const: Option<ParseableType>,
    source_get: Option<Get>,

    ord: Option<Vec<Ordering>>,
    
    state: State,
    
    none_check: bool,
    
    not: bool,
    len: bool,
}

impl Default for IfBuilder {
    fn default() -> Self {
        IfBuilder {
            target: vec![],
            target_data_type: BfpType::Int8(Int8),
            source: None,
            source_const: None,
            source_get: None,
            min_ver: None,
            max_ver: None,
            ord: None,
            state: State::HasTarget,
            not: false,
            len: false,
            none_check: false,
        }
    }
}

impl IfBuilder {
    pub fn cmp_path(&mut self, source: &Bound<PyTuple>, ord: Vec<Ordering>) -> PyResult<()> {
        let (source, _source_data_type, _source_name) = idxes_from_tup(source)?;
        
        self.source = Some(source);
        self.ord = Some(ord);
        self.state = State::HasSource;
        Ok(())
    }
    pub fn cmp_fix<'py>(&mut self, source: &Bound<PyAny>, ord: Vec<Ordering>) -> PyResult<()> {
        if self.len {
            let val2 = source.extract::<isize>()?;
            if val2 < 0 {
                return Err(PyValueError::new_err(
                    "Using a negative value in a length comparison is a bug"
                ))
            }
            self.source = Some(vec![val2 as usize]);
        } else {
            self.source_const = Some(self.target_data_type.to_parseable(&source)?)
        };
        self.ord = Some(ord);
        self.state = State::HasSourceConst;
        
        Ok(())
    }
    
    pub fn cmp_get<'py>(&mut self, mut source: Get, ord: Vec<Ordering>) -> PyResult<()> {
        source.make_contiguous();
        self.source_get = Some(source);
        self.ord = Some(ord);
        self.state = State::HasSourceGet;
        Ok(())
    }
    
    pub fn cmp(&mut self, source: &Bound<PyTuple>, ord: Vec<Ordering>) -> PyResult<()> {
        if self.state != State::HasTarget {
            return Err(PyTypeError::new_err(
                "Cannot chain comparisons, use a .then() with a nested if_"
            ))
        }
        let len = <Bound<PyTuple> as PyTupleMethods>::len(source);

        if len == 1 {
            let item = unsafe { source.get_item_unchecked(0) };
            if let Ok(_ret) = item.downcast::<Retriever>() {
                self.cmp_path(source, ord)
            } else if let Ok(get) = item.extract::<Get>() {
                self.cmp_get(get, ord)
            } else {
                self.cmp_fix(&item, ord)
            }
        } else {
            self.cmp_path(source, ord)
        }
    }
}

#[pymethods]
impl IfBuilder {
    fn is_none<'py>(slf: Bound<'py, Self>) -> Bound<'py, Self> {
        slf.borrow_mut().none_check = true;
        slf
    }

    fn then(&self, com: CombinatorType) -> PyResult<CombinatorType> {
        Ok(match self.state {
            State::VerCheck => {
                IfVer::new(
                    self.min_ver.as_ref().expect("infallible"),
                    self.max_ver.as_ref().expect("infallible"),
                    com,
                ).into()
            }
            State::HasTarget if self.none_check => {
                IfIsNone::new(
                    &self.target,
                    com,
                    self.not,
                ).into()
            }
            State::HasTarget => {
                IfCheck::new(
                    &self.target,
                    com,
                    self.not,
                ).into()
            }
            State::HasSource if self.len => {
                IfCmpLenFrom::new(
                    &self.target,
                    self.source.as_ref().expect("infallible"),
                    self.ord.as_ref().expect("infallible"),
                    com,
                ).into()
            }
            State::HasSource => {
                IfCmpFrom::new(
                    &self.target,
                    self.source.as_ref().expect("infallible"),
                    self.ord.as_ref().expect("infallible"),
                    com,
                ).into()
            }
            State::HasSourceConst if self.len => {
                IfCmpLenTo::new(
                    &self.target,
                    self.source.as_ref().expect("infallible")[0],
                    self.ord.as_ref().expect("infallible"),
                    com,
                ).into()
            }
            State::HasSourceConst => {
                IfCmpTo::new(
                    &self.target,
                    self.source_const.as_ref().expect("infallible"),
                    self.ord.as_ref().expect("infallible"),
                    com,
                ).into()
            }
            State::HasSourceGet if self.len => {
                IfCmpLenBy::new(
                    &self.target,
                    self.source_get.as_ref().expect("infallible"),
                    self.ord.as_ref().expect("infallible"),
                    com,
                ).into()
            }
            State::HasSourceGet => {
                IfCmpBy::new(
                    &self.target,
                    self.source_get.as_ref().expect("infallible"),
                    self.ord.as_ref().expect("infallible"),
                    com,
                ).into()
            }
        })
    }

    #[pyo3(signature = (*source), text_signature = "(*source: Retriever | int | Get)")]
    fn eq<'py>(slf: Bound<'py, Self>, source: &Bound<PyTuple>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(source, vec![Ordering::Less, Ordering::Greater])?;
        } else {
            this.cmp(source, vec![Ordering::Equal])?;
        }
        Ok(slf)
    }

    #[pyo3(signature = (*source), text_signature = "(*source: Retriever | int | Get)")]
    fn ne<'py>(slf: Bound<'py, Self>, source: &Bound<PyTuple>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(source, vec![Ordering::Equal])?;
        } else {
            this.cmp(source, vec![Ordering::Less, Ordering::Greater])?;
        }
        Ok(slf)
    }

    #[pyo3(signature = (*source), text_signature = "(*source: Retriever | int | Get)")]
    fn gt<'py>(slf: Bound<'py, Self>, source: &Bound<PyTuple>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(source, vec![Ordering::Less, Ordering::Equal])?;
        } else {
            this.cmp(source, vec![Ordering::Greater])?;
        }
        Ok(slf)
    }

    #[pyo3(signature = (*source), text_signature = "(*source: Retriever | int | Get)")]
    fn ge<'py>(slf: Bound<'py, Self>, source: &Bound<PyTuple>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(source, vec![Ordering::Less])?;
        } else {
            this.cmp(source, vec![Ordering::Greater, Ordering::Equal])?;
        }
        Ok(slf)
    }

    #[pyo3(signature = (*source), text_signature = "(*source: Retriever | int | Get)")]
    fn lt<'py>(slf: Bound<'py, Self>, source: &Bound<PyTuple>) -> PyResult<Bound<'py, Self>> {
        let mut this = slf.borrow_mut();
        if this.not {
            this.cmp(source, vec![Ordering::Greater, Ordering::Equal])?;
        } else {
            this.cmp(source, vec![Ordering::Less])?;
        }
        Ok(slf)
    }
    
    #[pyo3(signature = (*source), text_signature = "(*source: Retriever | int | Get)")]
    fn le<'py>(slf: Bound<'py, Self>, source: &Bound<PyTuple>) -> PyResult<Bound<'py, Self>> {
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
#[pyo3(signature = (*target), text_signature = "(*source: Retriever | int | Get)")]
pub fn if_(target: &Bound<PyTuple>) -> PyResult<IfBuilder> {
    let (target, target_data_type, _target_name) = idxes_from_tup(target)?;
    
    Ok(IfBuilder {
        target,
        target_data_type,
        ..Default::default()
    })
}

#[pyfunction]
#[pyo3(signature = (*target), text_signature = "(*source: Retriever | int | Get)")]
pub fn if_not(target: &Bound<PyTuple>) -> PyResult<IfBuilder> {
    let (target, target_data_type, _target_name) = idxes_from_tup(target)?;
    
    Ok(IfBuilder {
        target,
        target_data_type,
        not: true,
        ..Default::default()
    })
}

#[pyfunction]
#[pyo3(signature = (*target), text_signature = "(*source: Retriever | int | Get)")]
pub fn if_len(target: &Bound<PyTuple>) -> PyResult<IfBuilder> {
    let (target, target_data_type, _target_name) = idxes_from_tup(target)?;
    
    Ok(IfBuilder {
        target,
        target_data_type,
        len: true,
        ..Default::default()
    })
}

#[pyfunction]
#[pyo3(signature = (*, min = Version::new(vec![-1]), max = Version::new(vec![10_000])), text_signature = "(*, min: Version = Version(-1), max: Version = Version(10_000))")]
pub fn if_ver(min: Version, max: Version) -> PyResult<IfBuilder> {
    Ok(IfBuilder {
        min_ver: Some(min),
        max_ver: Some(max),
        state: State::VerCheck,
        ..Default::default()
    })
}
