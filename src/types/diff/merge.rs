use pyo3::{Py, PyAny, PyResult, Python};
use crate::types::diff::diff::{Diff, Diffable, IDiff};
use crate::types::diff::struct_mergeable::StructMergeable;
use crate::types::merge_py::{BasicPy, NestedConflictPy};
use crate::types::parseable_type::ParseableType;

pub trait Mergeable<T>: Diffable<T> {
    fn patch(&mut self, change: IDiff<T>, off: isize) -> isize;
    fn merge(&mut self, slf: &Self, other: &Self) -> Vec<Conflict<T>>;
}

#[derive(Debug, Clone)]
pub enum Conflict<T> {
    Basic(usize, Diff<T>, Diff<T>),
    Nested(usize, Vec<Conflict<T>>)
}

impl<T> Conflict<T> {
    pub fn idx(&self) -> usize {
        match self {
            Conflict::Basic(i, _, _) => *i,
            Conflict::Nested(i, _) => *i,
        }
    }
}

impl Conflict<ParseableType> {
    pub fn to_pyobj(self, old: Option<&mut ParseableType>, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match self {
            Conflict::Basic(_, change1, change2) => {
                let old = old.as_deref();
                Ok(BasicPy {
                    old: old.cloned()
                        .map(|val| val.to_bound(py))
                        .transpose()?
                        .map(|val| val.unbind()),
                    change1: change1.to_pyobj(old, py)?,
                    change2: change2.to_pyobj(old, py)?,
                }.into_pyany(py))
            }
            Conflict::Nested(_i, conflicts) => {
                let val = old.expect("Conflict::Nested: Merging structs of different versions is not allowed");
                match val {
                    ParseableType::Struct { val, struct_ } => {
                        Ok(NestedConflictPy {
                            children: StructMergeable(struct_, val).to_dict(conflicts, py)?.unbind()
                        }.into_pyany(py))
                    }
                    ParseableType::Array(ls) => {
                        Ok(NestedConflictPy {
                            children: ls.conflicts_to_dict(conflicts, py)?.unbind()
                        }.into_pyany(py))
                    }
                    ParseableType::Option(val) => {
                        Conflict::Nested(_i, conflicts).to_pyobj(val.as_deref_mut(), py)
                    }
                    _ => { unreachable!("Conflict::Nested cannot be created with non-nested data") }
                }
            }
        }
    }
}