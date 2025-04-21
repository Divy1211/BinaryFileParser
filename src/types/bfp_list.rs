use std::cmp::Ordering;
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};
use std::sync::RwLock;

use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::{PyAnyMethods, PyTypeMethods};
use pyo3::types::{PyInt, PySlice, PySliceIndices, PySliceMethods};
use pyo3::{pyclass, pymethods, Bound, IntoPy, PyAny, PyRef, PyRefMut, PyResult};
use crate::errors::mutability_error::MutabilityError;
use crate::types::bfp_type::BfpType;
use crate::types::parseable_type::ParseableType;

#[derive(Debug)]
pub struct BfpListRaw {
    pub data: Vec<ParseableType>,
    pub data_type: BfpType,
    pub immutable: bool,
}

#[pyclass(sequence, eq)]
#[derive(Debug, Clone)]
pub struct BfpList {
    raw: Arc<RwLock<BfpListRaw>>,
}

impl BfpList {
    pub fn new(data: Vec<ParseableType>, data_type: BfpType) -> BfpList {
        BfpList { raw: Arc::new(RwLock::new(BfpListRaw {
            data,
            data_type,
            immutable: false,
        }))}
    }
    
    pub fn inner(&self) -> RwLockReadGuard<BfpListRaw> {
        self.raw.read().expect("GIL Bound read")
    }

    pub fn inner_mut(&self) -> RwLockWriteGuard<BfpListRaw> {
        self.raw.write().expect("GIL Bound read")
    }
    
    pub fn len(&self) -> usize {
        self.raw.read().expect("GIL Bound read").data.len()
    }
}

impl PartialOrd for BfpList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let data1 = &self.raw.read().expect("GIL bound read").data;
        let data2 = &other.raw.read().expect("GIL bound read").data;
        
        data1.partial_cmp(data2)
    }
}

impl PartialEq for BfpList {
    fn eq(&self, other: &Self) -> bool {
        let data1 = &self.raw.read().expect("GIL bound read").data;
        let data2 = &other.raw.read().expect("GIL bound read").data;
        if data1.len() != data2.len() {
            return false
        }
        data1.iter().zip(data2.iter())
            .map(|(a, b)| a == b)
            .all(|x| x)
    }
}

impl Eq for BfpList {}

#[pymethods]
impl BfpList {
    fn append(slf: PyRefMut<BfpList>, val: Bound<'_, PyAny>) -> PyResult<()> {
        let mut inner = slf.inner_mut();

        if inner.immutable {
            return Err(MutabilityError::new_err("This list is set as immutable by it's API designer"));
        }
        let val = inner.data_type.to_parseable(&val)?;
        inner.data.push(val);
        Ok(())
    }

    fn extend(slf: PyRefMut<BfpList>, val: Bound<'_, PyAny>) -> PyResult<()> {
        let mut inner = slf.inner_mut();
        
        if inner.immutable {
            return Err(MutabilityError::new_err("This list is set as immutable by it's API designer"));
        }

        let mut vals = val.iter()?
            .map(|v| {
                inner.data_type.to_parseable(&v.expect("obtained from python"))
            })
            .collect::<PyResult<Vec<_>>>()?;
        
        inner.data.append(&mut vals);
        Ok(())
    }

    fn insert(slf: PyRefMut<BfpList>, mut item: isize, val: Bound<'_, PyAny>) -> PyResult<()> {
        let mut inner = slf.inner_mut();
        
        if inner.immutable {
            return Err(MutabilityError::new_err("This list is set as immutable by it's API designer"));
        }
        
        if item < 0 {
            item += inner.data.len() as isize;
        }
        let item = item.clamp(0, inner.data.len() as isize) as usize;
        
        let val = inner.data_type.to_parseable(&val)?;
        inner.data.insert(item, val);
        Ok(())
    }

    /// note that remove in python takes a value, not an index
    fn remove(slf: PyRefMut<BfpList>, val: Bound<'_, PyAny>) -> PyResult<()> {
        let mut inner = slf.inner_mut();
        
        if inner.immutable {
            return Err(MutabilityError::new_err("This list is set as immutable by it's API designer"));
        }
        let val = inner.data_type.to_parseable(&val)?;
        
        let idx = match inner.data.iter().position(|x| *x == val) {
            Some(idx) => { idx }
            None => {
                return Err(PyValueError::new_err("list.remove(x): x not in list"))
            }
        };
        
        inner.data.remove(idx);
        Ok(())
    }

    #[pyo3(signature = (item = -1))]
    fn pop<'py>(slf: PyRefMut<'py, BfpList>, mut item: isize) -> PyResult<Bound<'py, PyAny>> {
        let mut inner = slf.inner_mut();
        
        if inner.immutable {
            return Err(MutabilityError::new_err("This list is set as immutable by it's API designer"));
        }
        
        if item < 0 {
            item += inner.data.len() as isize;
        }
        if item < 0 || item >= inner.data.len() as isize {
            return Err(PyIndexError::new_err("list index out of range"))
        }
        
        Ok(inner.data.remove(item as usize).to_bound(slf.py()))
    }

    fn clear<'py>(slf: PyRefMut<'py, BfpList>) -> PyResult<()> {
        let mut inner = slf.inner_mut();
        
        if inner.immutable {
            return Err(MutabilityError::new_err("This list is set as immutable by it's API designer"));
        }
        inner.data.clear();

        Ok(())
    }

    fn index(slf: PyRef<BfpList>, val: Bound<'_, PyAny>) -> PyResult<usize> {
        let inner = slf.inner();

        if inner.immutable {
            return Err(MutabilityError::new_err("This list is set as immutable by it's API designer"));
        }
        let val = inner.data_type.to_parseable(&val)?;
        
        match inner.data.iter().position(|x| *x == val) {
            Some(idx) => { Ok(idx) }
            None => {
                Err(PyValueError::new_err("list.index(x): x not in list"))
            }
        }
    }

    fn count(slf: PyRef<BfpList>, val: Bound<'_, PyAny>) -> PyResult<usize> {
        let inner = slf.inner();
        let val = inner.data_type.to_parseable(&val)?;
        Ok(
            inner.data.iter()
                .filter(|x| **x == val)
                .count()
        )
    }

    fn reverse(slf: PyRefMut<BfpList>) -> PyResult<()> {
        let mut inner = slf.inner_mut();
        
        if inner.immutable {
            return Err(MutabilityError::new_err("This list is set as immutable by it's API designer"));
        }
        inner.data.reverse();
        Ok(())
    }
    
    fn sort(slf: PyRefMut<BfpList>) -> PyResult<()> {
        let mut inner = slf.inner_mut();
        
        if inner.immutable {
            return Err(MutabilityError::new_err("This list is set as immutable by it's API designer"));
        }

        if !inner.data_type.is_ord() {
            return Err(PyTypeError::new_err(format!(
                "Can't sort list because comparing instances of '{}' is not supported",
                inner.data_type.py_name()
            )));
        }
        inner.data.sort_by(|a, b| a.partial_cmp(b).expect("Bfp Internal Error: BfpType::is_ord is bugged"));
        Ok(())
    }

    fn copy(slf: PyRef<BfpList>) -> Self {
        slf.clone()
    }

    fn __len__(&self) -> usize {
        self.len()
    }

    fn __getitem__<'py>(slf: PyRef<'py, BfpList>, item: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let inner = slf.inner();
        
        if item.is_instance_of::<PyInt>() {
            let mut item = item.extract::<isize>().expect("infallible");
            if item < 0 {
                if item < -(inner.data.len() as isize) {
                    return Err(PyIndexError::new_err("list index out of range"))
                }
                item += inner.data.len() as isize;
            }
            let item = item as usize;
            
            if item >= inner.data.len() {
                return Err(PyIndexError::new_err("list index out of range"))
            }
            
            return Ok(
                inner.data[item]
                    .clone()
                    .to_bound(slf.py())
            );
        }
        if item.is_instance_of::<PySlice>() {
            let item = item.downcast_into::<PySlice>().expect("infallible");
            let idxes = slice(item.indices(inner.data.len() as isize)?)?;
            
            return Ok(
                idxes.into_iter()
                    .map(|idx| inner.data[idx].clone().to_bound(slf.py()) )
                    .collect::<Vec<_>>()
                    .into_py(slf.py())
                    .into_bound(slf.py())
            )
        }
        Err(PyIndexError::new_err(
            format!("list indices must be integers or slices, not '{}'", item.get_type().fully_qualified_name()?.to_string())
        ))
    }

    fn __setitem__(slf: PyRefMut<BfpList>, item: Bound<PyAny>, val: Bound<PyAny>) -> PyResult<()> {
        let mut inner = slf.inner_mut();
        
        if inner.immutable {
            return Err(MutabilityError::new_err("This list is set as immutable by it's API designer"));
        }
        if item.is_instance_of::<PyInt>() {
            let item = item.extract::<usize>().expect("infallible");
            if item >= inner.data.len() {
                return Err(PyIndexError::new_err("list index out of range"))
            }
            
            inner.data[item] = inner.data_type.to_parseable(&val)?;
            return Ok(())
        }
        if item.is_instance_of::<PySlice>() {
            let item = item.downcast_into::<PySlice>().expect("infallible");
            let idxes = slice(item.indices(inner.data.len() as isize)?)?;

            let vals = val.iter()?
                .map(|v| v.expect("obtained from python"))
                .collect::<Vec<_>>();
            if idxes.len() != vals.len() {
                return Err(PyValueError::new_err(
                    format!("attempt to assign sequence of size {} to slice of size {}", vals.len(), idxes.len())
                ))
            }
            
            for (idx, val) in idxes.into_iter().zip(vals) {
                inner.data[idx] = inner.data_type.to_parseable(&val)?;
            }
            return Ok(())
        }
        Err(PyIndexError::new_err(
            format!("list indices must be integers or slices, not '{}'", item.get_type().fully_qualified_name()?.to_string())
        ))
    }

    fn __delitem__(slf: PyRefMut<BfpList>, item: Bound<PyAny>) -> PyResult<()> {
        let mut inner = slf.inner_mut();

        if inner.immutable {
            return Err(MutabilityError::new_err("This list is set as immutable by it's API designer"));
        }
        if item.is_instance_of::<PyInt>() {
            let item = item.extract::<usize>().expect("infallible");
            if item >= inner.data.len() {
                return Err(PyIndexError::new_err("list index out of range"))
            }
            
            inner.data.remove(item);
            return Ok(())
        }
        if item.is_instance_of::<PySlice>() {
            let item = item.downcast_into::<PySlice>().expect("infallible");
            let idxes = slice(item.indices(inner.data.len() as isize)?)?;
            
            for i in idxes.into_iter().rev() {
                inner.data.remove(i);
            }
            
            return Ok(())
        }
        Err(PyIndexError::new_err(
            format!("list indices must be integers or slices, not '{}'", item.get_type().fully_qualified_name()?.to_string())
        ))
    }

    fn __repr__(slf: PyRef<BfpList>) -> String { // todo: implement this properly
        let inner = slf.inner();

        format!(
            "[{}]",
            inner.data.iter()
                .map(|l| l.clone().to_bound(slf.py()).to_string())
                .collect::<Vec<String>>().join(", ")
        )
    }
}

fn slice(slice: PySliceIndices) -> PyResult<Vec<usize>> {
    let (start, stop, step) = (slice.start as usize, slice.stop as usize, slice.step);
    if step == 0 {
        return Err(PyValueError::new_err("slice step cannot be zero"));
    }

    let idxes: Vec<usize> = if step > 0 {
        (start..stop).step_by(step as usize).collect()
    } else {
        (stop+1..start-1).rev().step_by((-step) as usize).collect()
    };
    
    Ok(idxes)
}
