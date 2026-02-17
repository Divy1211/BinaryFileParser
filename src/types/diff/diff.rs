use pyo3::{Python, PyResult, PyAny, Py};
use crate::types::diff::struct_diffable::StructDiffable;
use crate::types::diff_py::{ChangedPy, DeletedPy, InsertedPy, NestedDiffPy};
use crate::types::parseable_type::ParseableType;

#[derive(Debug, Clone)]
pub enum Diff<T> {
    None,
    Inserted(T),
    Deleted(T),
    Changed(T),
    Nested(Vec<IDiff<T>>)
}

impl<T> Diff<T> {
    pub fn value(self) -> Option<T> {
        match self {
            Diff::None => None,
            Diff::Inserted(v) | Diff::Deleted(v) | Diff::Changed(v) => Some(v),
            Diff::Nested(_) => { unreachable!("BFP Internal Error: Attempted to extract value from nested diff") }
        }
    }
}

pub trait Diffable<T> : Sized {
    fn diff(&self, other: &Self) -> Diff<T>;
}

pub type IDiff<T> = (usize, Diff<T>);

impl Diff<ParseableType> {
    pub fn to_pyobj(self, old: Option<&ParseableType>, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match self {
            Diff::None => { Ok(py.None()) }
            Diff::Inserted(val) => {
                Ok(InsertedPy { value: val.to_bound(py)?.unbind() }.into_pyany(py))
            }
            Diff::Deleted(val) => {
                Ok(DeletedPy { value: val.to_bound(py)?.unbind() }.into_pyany(py))
            }
            Diff::Changed(val) => {
                Ok(ChangedPy {
                    old: old.cloned()
                        .expect("Diff::Changed cannot be created with unsupported attributes")
                        .to_bound(py)?.unbind(),
                    new: val.to_bound(py)?.unbind()
                }.into_pyany(py))
            }
            Diff::Nested(changes) => {
                let val = old.expect("Diff::Changed cannot be created with unsupported attributes");
                match val {
                    ParseableType::Struct { val, struct_ } => {
                        Ok(NestedDiffPy {
                            children: StructDiffable(struct_, val).to_dict(Diff::Nested(changes), py)?.unbind()
                        }.into_pyany(py))
                    }
                    ParseableType::Array(ls) => {
                        Ok(NestedDiffPy {
                            children: ls.diffs_to_dict(changes, py)?.unbind()
                        }.into_pyany(py))
                    }
                    ParseableType::Option(val) => {
                        Diff::Nested(changes).to_pyobj(val.as_deref(), py)
                    }
                    _ => { unreachable!("Diff::Nested cannot be created with non-nested data") }
                }
            }
        }
    }
}