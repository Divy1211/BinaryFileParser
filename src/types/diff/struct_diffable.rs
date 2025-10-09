use crate::types::base_struct::BaseStruct;
use crate::types::diff::{Diff, Diffable};
use crate::types::parseable_type::ParseableType;
use crate::types::r#struct::Struct;

pub struct StructDiffable<'a, 'b>(pub &'a Struct, pub &'b BaseStruct);

fn expect_err<'a>(val: Option<&'a ParseableType>, name: &str) -> &'a ParseableType {
    val.expect(&format!("Diffing uninitialized value '{name}'"))
}

impl Diffable for StructDiffable<'_, '_> {
    type DiffResult = Vec<(usize, Diff<ParseableType>)>;

    fn diff(&self, other: &Self) -> Self::DiffResult {
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
                    
                    let Some(val_diff) = val1.diff(val2) else {
                        continue;
                    };
                    diff.push((i, val_diff));
                },
            };
        }
        
        diff
    }
}
