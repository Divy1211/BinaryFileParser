use pyo3::{Bound, PyResult, Python};
use pyo3::prelude::PyDictMethods;
use pyo3::types::PyDict;

use crate::types::base_struct::BaseStruct;
use crate::types::diff::diff::{Diff, Diffable, IDiff};
use crate::types::diff::merge::{Conflict, Mergeable};
use crate::types::diff::struct_diffable::StructDiffable;
use crate::types::parseable_type::ParseableType;
use crate::types::r#struct::Struct;

pub struct StructMergeable<'a, 'b>(pub &'a Struct, pub &'b mut BaseStruct);

impl Diffable<ParseableType> for StructMergeable<'_, '_> {
    fn diff(&self, other: &Self) -> Diff<ParseableType> {
        StructDiffable(self.0, self.1).diff(&StructDiffable(other.0, other.1))
    }
}

impl Mergeable<ParseableType> for StructMergeable<'_, '_> {
    fn patch(
        &mut self,
        (idx, change): IDiff<ParseableType>,
        _off: isize
    ) -> isize {
        let mut inner = self.1.inner_mut();
        let data = &mut inner.data;
        match change {
            Diff::None => _off,
            Diff::Inserted(_val) | Diff::Deleted(_val) => {
                unreachable!("BFP Internal Error: merge called on structs of different versions")
            }
            Diff::Changed(val) => {
                data[idx] = Some(val);
                _off
            }
            Diff::Nested(changes) => {
                let val = data[idx].as_mut()
                    .expect("Unsupported attributes are always inserted or deleted");
                let mut off = 0;
                for change in changes {
                    off = val.patch(change, off);
                }
                _off
            }
        }
    }

    fn merge(&mut self, slf: &Self, other: &Self) -> Vec<Conflict<ParseableType>> {
        let diff1 = self.diff(slf);
        let diff2 = self.diff(other);

        let (changes1, changes2) = match (diff1, diff2) {
            (Diff::None, Diff::None) => return vec![],
            (Diff::Nested(changes), Diff::None) | (Diff::None, Diff::Nested(changes)) => {
                for change in changes {
                    self.patch(change, 0);
                }
                return vec![];
            }
            (Diff::Nested(changes1), Diff::Nested(changes2)) => (changes1, changes2),
            _ => unreachable!("BFP Internal Error: merge called on non-nested diff")
        };

        let mut conflicts = vec![];
        self.merge_rec(changes1, changes2, &mut conflicts);
        conflicts
    }
}

impl StructMergeable<'_, '_> {
    pub fn merge_rec(
        &mut self,
        changes1: Vec<IDiff<ParseableType>>,
        changes2: Vec<IDiff<ParseableType>>,
        conflicts: &mut Vec<Conflict<ParseableType>>,
    ) {
        let (mut it1, mut it2) = (changes1.into_iter(), changes2.into_iter());
        let (mut e1, mut e2) = (it1.next(), it2.next());
        
        loop { match (e1, e2) { (None, None) => break,
            (Some(diff), None) | (None, Some(diff)) => {
                self.patch(diff, isize::MIN);
                (e1, e2) = (it1.next(), it2.next());
            }
            (Some(diff1), Some(diff2)) => {
                #[allow(clippy::comparison_chain)]
                if diff1.0 == diff2.0 {
                    let (idx, change1) = diff1;
                    let change2 = diff2.1;
                    match (change1, change2) {
                        (Diff::Changed(val1), Diff::Changed(val2)) => {
                            if val1 == val2 {
                                self.patch((idx, Diff::Changed(val1)), isize::MIN);
                            } else {
                                conflicts.push(Conflict::Basic(idx, Diff::Changed(val1), Diff::Changed(val2)));
                            }
                        }
                        (Diff::Nested(sub_changes1), Diff::Nested(sub_changes2)) => {
                            let mut inner = self.1.inner_mut();
                            let data = &mut inner.data;
                            
                            let val = data[idx].as_mut()
                                .expect("Unsupported attributes are always inserted or deleted");
                            
                            let mut sub_conflicts = vec![];
                            val.merge_rec(sub_changes1, sub_changes2, &mut sub_conflicts);
                            if !sub_conflicts.is_empty() {
                                conflicts.push(Conflict::Nested(idx, sub_conflicts));
                            }
                        }
                        // Diff::Deleted, Diff::Inserted - never occur. merging structs of different versions is not allowed
                        // Diff::None - no op
                        _ => {}
                    }
                    (e1, e2) = (it1.next(), it2.next());
                } else if diff1.0 < diff2.0 {
                    self.patch(diff1, isize::MIN);
                    (e1, e2) = (it1.next(), Some(diff2));
                } else {
                    self.patch(diff2, isize::MIN);
                    (e1, e2) = (Some(diff1), it2.next());
                }
            }
        }}
    }
}


impl StructMergeable<'_, '_> {
    pub fn to_dict<'py>(&self, conflicts: Vec<Conflict<ParseableType>>, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let retrievers = self.0.retrievers();
        let mut inner = self.1.inner_mut();

        let di = PyDict::new(py);
        if conflicts.is_empty() {
            return Ok(di);
        }
        for conflict in conflicts {
            let idx = conflict.idx();
            PyDictMethods::set_item(
                &di,
                &retrievers[idx].name,
                conflict.to_pyobj(inner.data[idx].as_mut(), py)?
            )?;
        }
        Ok(di)
    }
}