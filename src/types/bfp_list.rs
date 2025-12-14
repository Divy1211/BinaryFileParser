use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};
use std::sync::RwLock;

use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::{PyAnyMethods, PyDictMethods, PyTypeMethods};
use pyo3::types::{PyDict, PyInt, PySlice, PySliceIndices, PySliceMethods};
use pyo3::{pyclass, pymethods, Bound, IntoPyObjectExt, PyAny, PyRef, PyRefMut, PyResult, Python};
use serde::{Serialize, Serializer};
use crate::errors::mutability_error::MutabilityError;
use crate::types::bfp_type::BfpType;
use crate::types::diff::diff::{Diff, Diffable, IDiff};
use crate::types::diff::merge::{Conflict, Mergeable};
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

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.inner().data.len()
    }
}

impl PartialOrd for BfpList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let data1 = &self.inner().data;
        let data2 = &other.inner().data;
        
        data1.partial_cmp(data2)
    }
}

impl PartialEq for BfpList {
    fn eq(&self, other: &Self) -> bool {
        let data1 = &self.inner().data;
        let data2 = &other.inner().data;
        if data1.len() != data2.len() {
            return false
        }
        data1.iter().zip(data2.iter())
            .all(|(a, b)| a == b)
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

        let mut vals = val.try_iter()?
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
    fn pop(slf: PyRefMut<'_, BfpList>, mut item: isize) -> PyResult<Bound<'_, PyAny>> {
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
        
        inner.data.remove(item as usize).to_bound(slf.py())
    }

    fn clear(slf: PyRefMut<'_, BfpList>) -> PyResult<()> {
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
            
            return inner.data[item]
                .clone()
                .to_bound(slf.py());
        }
        if item.is_instance_of::<PySlice>() {
            let item = item.cast_into::<PySlice>().expect("infallible");
            let idxes = slice(item.indices(inner.data.len() as isize)?)?;
            
            return idxes.into_iter()
                    .map(|idx| inner.data[idx].clone().to_bound(slf.py()))
                    .collect::<PyResult<Vec<_>>>()?
                    .into_bound_py_any(slf.py())
        }
        Err(PyIndexError::new_err(
            format!("list indices must be integers or slices, not '{}'", item.get_type().fully_qualified_name()?)
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
            let item = item.cast_into::<PySlice>().expect("infallible");
            let idxes = slice(item.indices(inner.data.len() as isize)?)?;

            let vals = val.try_iter()?
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
            format!("list indices must be integers or slices, not '{}'", item.get_type().fully_qualified_name()?)
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
            let item = item.cast_into::<PySlice>().expect("infallible");
            let idxes = slice(item.indices(inner.data.len() as isize)?)?;
            
            for i in idxes.into_iter().rev() {
                inner.data.remove(i);
            }
            
            return Ok(())
        }
        Err(PyIndexError::new_err(
            format!("list indices must be integers or slices, not '{}'", item.get_type().fully_qualified_name()?)
        ))
    }

    fn __repr__(slf: PyRef<BfpList>) -> PyResult<String> { // todo: implement this properly
        let inner = slf.inner();

        Ok(format!(
            "[{}]",
            inner.data.iter()
                .map(|l| Ok(l.clone().to_bound(slf.py())?.to_string()))
                .collect::<PyResult<Vec<_>>>()?.join(", ")
        ))
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

impl Serialize for BfpList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner().data.serialize(serializer)
    }
}

impl Hash for BfpList {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().data.hash(state);
    }
}

impl Diffable<ParseableType> for BfpList {

    fn diff(&self, other: &Self) -> Diff<ParseableType> {
        let data1 = &self.inner().data;
        let data2 = &other.inner().data;
        data1.diff(data2)
    }
}

impl Mergeable<ParseableType> for BfpList {
    fn patch(&mut self, (idx, change): IDiff<ParseableType>, off: isize) -> isize {
        let mut inner = self.inner_mut();
        let ls = &mut inner.data;
        let idx = (idx as isize - off) as usize;

        match change {
            Diff::None => { off }
            Diff::Inserted(val) => { ls.insert(idx, val); off - 1 }
            Diff::Deleted(_val) => { ls.remove(idx); off + 1 }
            Diff::Changed(val) => { ls[idx] = val; off }
            Diff::Nested(changes) => {
                let val = &mut ls[idx];
                let mut off2 = 0;
                for change in changes {
                    off2 = val.patch(change, off2);
                }
                off
            }
        }
    }

    fn merge(&mut self, _slf: &Self, _other: &Self) -> Vec<Conflict<ParseableType>> {
        unreachable!("BFP Internal Error: merge called on BfpList")
    }
}

impl BfpList {
    pub fn merge_rec(
        &mut self,
        changes1: Vec<IDiff<ParseableType>>,
        changes2: Vec<IDiff<ParseableType>>,
        conflicts: &mut Vec<Conflict<ParseableType>>,
    ) {
        let (mut it1, mut it2) = (changes1.into_iter(), changes2.into_iter());
        let (mut e1, mut e2) = (it1.next(), it2.next());
        
        let mut off = 0;
        loop { match (e1, e2) { (None, None) => break,
            (Some(diff), None) | (None, Some(diff)) => {
                off = self.patch(diff, off);
                (e1, e2) = (it1.next(), it2.next());
            }
            (Some(diff1), Some(diff2)) => {
                #[allow(clippy::comparison_chain)]
                if diff1.0 == diff2.0 {
                    let (idx, change1) = diff1;
                    let change2 = diff2.1;
                    match (change1, change2) {
                        (Diff::Nested(sub_changes1), Diff::Nested(sub_changes2)) => {
                            let mut inner = self.inner_mut();
                            let ls = &mut inner.data;

                            let val = &mut ls[idx];

                            let mut sub_conflicts = vec![];
                            val.merge_rec(sub_changes1, sub_changes2, &mut sub_conflicts);
                            if !sub_conflicts.is_empty() {
                                conflicts.push(Conflict::Nested(idx, sub_conflicts));
                            }
                        }
                        (change1 @ Diff::Deleted(_), Diff::Deleted(_)) => {
                            off = self.patch((idx, change1), off);
                        }
                        (Diff::Changed(v1), Diff::Changed(v2)) if v1 == v2 => {
                            off = self.patch((idx, Diff::Changed(v1)), off);
                        }
                        (
                            change1 @ (Diff::Deleted(_) | Diff::Changed(_) | Diff::Nested(_)),
                            change2 @ (Diff::Deleted(_) | Diff::Changed(_) | Diff::Nested(_))
                        ) => {
                            conflicts.push(Conflict::Basic(idx, change1, change2));
                        }
                        (change1, change2) => {
                            off = self.patch((idx, change1), off);
                            off = self.patch((idx, change2), off);
                        }
                    }
                    (e1, e2) = (it1.next(), it2.next());
                } else if diff1.0 < diff2.0 {
                    off = self.patch(diff1, off);
                    (e1, e2) = (it1.next(), Some(diff2));
                } else {
                    off = self.patch(diff2, off);
                    (e1, e2) = (Some(diff1), it2.next());
                }
            }
        }}
    }
}

impl BfpList {
    pub fn diffs_to_dict<'py>(&self, changes: Vec<IDiff<ParseableType>>, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let di = PyDict::new(py);
        let inner = self.inner();
        for (idx, change) in changes {
            PyDictMethods::set_item(
                &di,
                idx,
                change.to_pyobj(inner.data.get(idx), py)?
            )?;
        }
        Ok(di)
    }

    pub fn conflicts_to_dict<'py>(&self, conflicts: Vec<Conflict<ParseableType>>, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let di = PyDict::new(py);
        let mut inner = self.inner_mut();
        for conflict in conflicts {
            let idx = conflict.idx();
            PyDictMethods::set_item(
                &di,
                idx,
                conflict.to_pyobj(inner.data.get_mut(idx), py)?
            )?;
        }
        Ok(di)
    }
}