use std::cmp::Ordering;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::RwLock;

use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::{PyAnyMethods, PyTypeMethods};
use pyo3::types::{PyInt, PySlice, PySliceIndices, PySliceMethods};
use pyo3::{pyclass, pymethods, Bound, IntoPy, PyAny, PyRef, PyRefMut, PyResult};

use crate::types::bfp_type::BfpType;
use crate::types::parseable_type::ParseableType;

#[pyclass(sequence, eq)]
#[derive(Debug, Clone)]
pub struct BfpList {
    pub ls: Arc<RwLock<Vec<ParseableType>>>,
    pub data_type: BfpType,
}

impl BfpList {
    pub fn new(ls: Vec<ParseableType>, data_type: BfpType) -> BfpList {
        BfpList {
            ls: Arc::new(RwLock::new(ls)),
            data_type
        }
    }
    
    pub fn len(&self) -> usize {
        self.ls.read().expect("GIL Bound read").len()
    }
}

impl PartialOrd for BfpList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ls1 = self.ls.read().expect("GIL bound read");
        let ls2 = other.ls.read().expect("GIL bound read");
        
        ls1.partial_cmp(&ls2)
    }
}

impl PartialEq for BfpList {
    fn eq(&self, other: &Self) -> bool {
        let ls1 = self.ls.read().expect("GIL bound read");
        let ls2 = other.ls.read().expect("GIL bound read");
        if ls1.len() != ls2.len() {
            return false
        }
        ls1.iter().zip(ls2.iter())
            .map(|(a, b)| a == b)
            .all(|x| x)
    }
}

impl Eq for BfpList {}

#[pymethods]
impl BfpList {
    fn append(slf: PyRefMut<BfpList>, val: Bound<'_, PyAny>) -> PyResult<()> {
        let mut ls = slf.ls.write().expect("GIL bound write");
        ls.push(slf.data_type.to_parseable(&val)?);
        Ok(())
    }

    fn extend(slf: PyRefMut<BfpList>, val: Bound<'_, PyAny>) -> PyResult<()> {
        let mut ls = slf.ls.write().expect("GIL bound write");
        let mut vals = val.iter()?
            .map(|v| {
                slf.data_type.to_parseable(&v.expect("obtained from python"))
            })
            .collect::<PyResult<Vec<_>>>()?;
        
        ls.append(&mut vals);
        Ok(())
    }

    fn insert(slf: PyRefMut<BfpList>, mut item: isize, val: Bound<'_, PyAny>) -> PyResult<()> {
        let mut ls = slf.ls.write().expect("GIL bound write");
        
        if item < 0 {
            item += ls.len() as isize;
        }
        let item = item.clamp(0, ls.len() as isize) as usize;
        
        ls.insert(item, slf.data_type.to_parseable(&val)?);
        Ok(())
    }

    /// note that remove in python takes a value, not an index
    fn remove(slf: PyRefMut<BfpList>, val: Bound<'_, PyAny>) -> PyResult<()> {
        let mut ls = slf.ls.write().expect("GIL bound write");
        let val = slf.data_type.to_parseable(&val)?;
        
        let idx = match ls.iter().position(|x| *x == val) {
            Some(idx) => { idx }
            None => {
                return Err(PyValueError::new_err("list.remove(x): x not in list"))
            }
        };
        
        ls.remove(idx);
        Ok(())
    }

    #[pyo3(signature = (item = -1))]
    fn pop<'py>(slf: PyRefMut<'py, BfpList>, mut item: isize) -> PyResult<Bound<'py, PyAny>> {
        let mut ls = slf.ls.write().expect("GIL bound write");
        
        if item < 0 {
            item += ls.len() as isize;
        }
        if item < 0 || item >= ls.len() as isize {
            return Err(PyIndexError::new_err("list index out of range"))
        }
        
        Ok(ls.remove(item as usize).to_bound(slf.py()))
    }

    fn clear<'py>(slf: PyRefMut<'py, BfpList>) -> PyResult<()> {
        let mut ls = slf.ls.write().expect("GIL bound write");
        ls.clear();
        Ok(())
    }

    fn index(slf: PyRefMut<BfpList>, val: Bound<'_, PyAny>) -> PyResult<usize> {
        let ls = slf.ls.read().expect("GIL bound read");
        let val = slf.data_type.to_parseable(&val)?;
        
        match ls.iter().position(|x| *x == val) {
            Some(idx) => { Ok(idx) }
            None => {
                Err(PyValueError::new_err("list.index(x): x not in list"))
            }
        }
    }

    fn count(slf: PyRefMut<BfpList>, val: Bound<'_, PyAny>) -> PyResult<usize> {
        let ls = slf.ls.read().expect("GIL bound read");
        let val = slf.data_type.to_parseable(&val)?;
        Ok(
            ls.iter()
                .filter(|x| **x == val)
                .count()
        )
    }

    fn reverse(slf: PyRefMut<BfpList>) -> PyResult<()> {
        let mut ls = slf.ls.write().expect("GIL bound write");
        ls.reverse();
        Ok(())
    }
    
    fn sort(slf: PyRefMut<BfpList>) -> PyResult<()> {
        let mut ls = slf.ls.write().expect("GIL bound write");
        if !slf.data_type.is_ord() {
            return Err(PyTypeError::new_err(format!(
                "Can't sort list because comparing instances of '{}' is not supported",
                slf.data_type.py_name()
            )));
        }
        ls.sort_by(|a, b| a.partial_cmp(b).expect("BfpType::is_ord is bugged"));
        Ok(())
    }

    fn copy(slf: PyRefMut<BfpList>) -> Self {
        slf.clone()
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.ls.read().expect("GIL bound read").len())
    }

    fn __getitem__<'py>(slf: PyRef<'py, BfpList>, item: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if item.is_instance_of::<PyInt>() {
            let ls = slf.ls.read().expect("GIL bound read");
            
            let mut item = item.extract::<isize>().expect("infallible");
            if item < 0 {
                if item < -(ls.len() as isize) {
                    return Err(PyIndexError::new_err("list index out of range"))
                }
                item += ls.len() as isize;
            }
            let item = item as usize;
            
            if item >= ls.len() {
                return Err(PyIndexError::new_err("list index out of range"))
            }
            
            return Ok(
                ls[item]
                    .clone()
                    .to_bound(slf.py())
            );
        }
        if item.is_instance_of::<PySlice>() {
            let ls = slf.ls.read().expect("GIL bound read");
            
            let item = item.downcast_into::<PySlice>().expect("infallible");
            let idxes = slice(item.indices(ls.len() as isize)?)?;
            
            return Ok(
                idxes.into_iter()
                    .map(|idx| ls[idx].clone().to_bound(slf.py()) )
                    .collect::<Vec<_>>()
                    .into_py(slf.py())
                    .into_bound(slf.py())
            )
        }
        Err(PyIndexError::new_err(
            format!("list indices must be integers or slices, not '{}'", item.get_type().fully_qualified_name()?.to_string())
        ))
    }

    fn __setitem__(slf: PyRef<BfpList>, item: Bound<PyAny>, val: Bound<PyAny>) -> PyResult<()> {
        if item.is_instance_of::<PyInt>() {
            let mut ls = slf.ls.write().expect("GIL bound write");
            
            let item = item.extract::<usize>().expect("infallible");
            if item >= ls.len() {
                return Err(PyIndexError::new_err("list index out of range"))
            }
            
            ls[item] = slf.data_type.to_parseable(&val)?;
            return Ok(())
        }
        if item.is_instance_of::<PySlice>() {
            let mut ls = slf.ls.write().expect("GIL bound write");
            
            let item = item.downcast_into::<PySlice>().expect("infallible");
            let idxs = slice(item.indices(ls.len() as isize)?)?;

            let vals = val.iter()?
                .map(|v| v.expect("obtained from python"))
                .collect::<Vec<_>>();
            if idxs.len() != vals.len() {
                return Err(PyValueError::new_err(
                    format!("attempt to assign sequence of size {} to slice of size {}", vals.len(), idxs.len())
                ))
            }
            
            for (idx, val) in idxs.into_iter().zip(vals) {
                ls[idx] = slf.data_type.to_parseable(&val)?;
            }
            return Ok(())
        }
        Err(PyIndexError::new_err(
            format!("list indices must be integers or slices, not '{}'", item.get_type().fully_qualified_name()?.to_string())
        ))
    }

    fn __delitem__(slf: PyRef<BfpList>, item: Bound<PyAny>) -> PyResult<()> {
        if item.is_instance_of::<PyInt>() {
            let mut ls = slf.ls.write().expect("GIL bound write");
            
            let item = item.extract::<usize>().expect("infallible");
            if item >= ls.len() {
                return Err(PyIndexError::new_err("list index out of range"))
            }
            
            ls.remove(item);
            return Ok(())
        }
        if item.is_instance_of::<PySlice>() {
            let mut ls = slf.ls.write().expect("GIL bound write");
            
            let item = item.downcast_into::<PySlice>().expect("infallible");
            let idxes = slice(item.indices(ls.len() as isize)?)?.into_iter().collect::<HashSet<_>>();
            
            *ls = ls.drain(..)
                .enumerate()
                .filter(|(idx, _)| !idxes.contains(&idx))
                .map(|(_, v)| v)
                .collect();
            
            return Ok(())
        }
        Err(PyIndexError::new_err(
            format!("list indices must be integers or slices, not '{}'", item.get_type().fully_qualified_name()?.to_string())
        ))
    }

    fn __repr__(slf: PyRef<BfpList>) -> String { // todo: implement this properly
        format!(
            "[{}]",
            slf.ls.read().expect("GIL bound read")
                .iter()
                .map(|l| l.clone().to_bound(slf.py()).to_string())
                .collect::<Vec<String>>().join(", ")
        )
    }
}

fn slice(slice: PySliceIndices) -> PyResult<Vec<usize>> {
    let (start, stop, step) = (slice.start, slice.stop, slice.step);
    if step == 0 {
        return Err(PyValueError::new_err("slice step cannot be zero"));
    }
    
    let mut idxes = Vec::with_capacity(((stop-start)/step).abs() as usize);
    let mut idx = start;
    if step < 0 { while idx > stop {
        idxes.push(idx as usize);
        idx += step;
    }} else { while idx < stop {
        idxes.push(idx as usize);
        idx += step;
    }}
    Ok(idxes)
}
