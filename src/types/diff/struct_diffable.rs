use pyo3::{Bound, PyResult, Python};
use pyo3::types::{PyDict, PyDictMethods};

use crate::types::base_struct::BaseStruct;
use crate::types::diff::diff::{Diff, Diffable};
use crate::types::parseable_type::ParseableType;
use crate::types::r#struct::Struct;

pub struct StructDiffable<'a, 'b>(pub &'a Struct, pub &'b BaseStruct);

fn expect_err<'a>(val: Option<&'a ParseableType>, name: &str) -> &'a ParseableType {
    val.unwrap_or_else(|| panic!("Diffing uninitialized value '{name}'"))
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
        if diff.is_empty() {
            return Diff::None;
        }
        Diff::Nested(diff)
    }
}

impl StructDiffable<'_, '_> {
    pub fn to_dict<'py>(&self, diff: Diff<ParseableType>, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let retrievers = self.0.retrievers();
        let inner = self.1.inner();

        let di = PyDict::new(py);
        let Diff::Nested(diff) = diff else {
            return Ok(di);
        };
        for (idx, change) in diff {
            PyDictMethods::set_item(
                &di,
                &retrievers[idx].name,
                change.to_pyobj(inner.data[idx].as_ref(), py)?
            )?;
        }
        Ok(di)
    }
}