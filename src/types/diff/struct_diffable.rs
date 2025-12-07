use pyo3::{Bound, IntoPy, PyResult, Python};
use pyo3::types::{PyDict, PyDictMethods};

use crate::types::base_struct::BaseStruct;
use crate::types::diff::diff::{Diff, Diffable};
use crate::types::diff_py::{ChangedPy, DeletedPy, InsertedPy, NestedDiffPy};
use crate::types::parseable_type::ParseableType;
use crate::types::r#struct::Struct;

pub struct StructDiffable<'a, 'b>(pub &'a Struct, pub &'b BaseStruct);

fn expect_err<'a>(val: Option<&'a ParseableType>, name: &str) -> &'a ParseableType {
    val.expect(&format!("Diffing uninitialized value '{name}'"))
}

impl Diffable<ParseableType> for StructDiffable<'_, '_> {
    fn diff(&self, other: &Self) -> Diff<ParseableType> {
        let struct1 = self.0;

        let inner1 = self.1.inner();
        let inner2 = other.1.inner();
        
        let retrievers = struct1.retrievers();

        let mut diff = Vec::with_capacity(retrievers.len());
        
        for (i, retriever) in retrievers.iter().enumerate() {
            match (retriever.supported(&inner1.ver), retriever.supported(&inner2.ver)) {
                (false, false) => {},
                (true, false) => {
                    let val1 = expect_err(inner1.data[i].as_ref(), &retriever.name);
                    diff.push((i, Diff::Deleted(val1.clone())));
                },
                (false, true) => {
                    let val2 = expect_err(inner2.data[i].as_ref(), &retriever.name);
                    diff.push((i, Diff::Inserted(val2.clone())));
                },
                (true, true) => {
                    let val1 = expect_err(inner1.data[i].as_ref(), &retriever.name);
                    let val2 = expect_err(inner2.data[i].as_ref(), &retriever.name);
                    let result = val1.diff(val2);
                    if let Diff::None = result {
                        continue;
                    };
                    diff.push((i, result));
                },
            };
        }
        if diff.len() == 0 {
            return Diff::None;
        }
        Diff::Nested(diff)
    }
}

impl StructDiffable<'_, '_> {
    pub fn to_dict<'py>(&self, diff: Diff<ParseableType>, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let retrievers = self.0.retrievers();
        let obj = self.1.inner();

        let di = PyDict::new_bound(py);
        let Diff::Nested(diff) = diff else {
            return Ok(di);
        };
        for (idx, change) in diff {
            match change {
                Diff::None => {}
                Diff::Inserted(val) => {
                    PyDictMethods::set_item(
                        &di,
                        &retrievers[idx].name,
                        InsertedPy { value: val.to_bound(py)?.unbind() }.into_py(py)
                    )?;
                }
                Diff::Deleted(val) => {
                    PyDictMethods::set_item(
                        &di,
                        &retrievers[idx].name,
                        DeletedPy { value: val.to_bound(py)?.unbind() }.into_py(py)
                    )?;
                }
                Diff::Changed(val) => {
                    PyDictMethods::set_item(
                        &di,
                        &retrievers[idx].name,
                        ChangedPy {
                            old: obj.data[idx].clone()
                                .expect("Diff::Changed cannot be created with unsupported attributes")
                                .to_bound(py)?.unbind(),
                            new: val.to_bound(py)?.unbind()
                        }.into_py(py)
                    )?;
                }
                Diff::Nested(changes) => {
                    let val = obj.data[idx].as_ref()
                        .expect("Diff::Changed cannot be created with unsupported attributes");
                    match val {
                        ParseableType::Struct { val, struct_ } => {
                            PyDictMethods::set_item(
                                &di,
                                &retrievers[idx].name,
                                NestedDiffPy {
                                    children: StructDiffable(struct_, val).to_dict(Diff::Nested(changes), py)?.unbind()
                                }.into_py(py)
                            )?;
                        }
                        ParseableType::Array(ls) => {
                            PyDictMethods::set_item(
                                &di,
                                &retrievers[idx].name,
                                NestedDiffPy {
                                    children: ls.diffs_to_dict(changes, py)?.unbind()
                                }.into_py(py)
                            )?;
                        }
                        _ => { unreachable!("Diff::Nested cannot be created with non-nested data") }
                    }
                }
            }
        }
        Ok(di)
    }
}