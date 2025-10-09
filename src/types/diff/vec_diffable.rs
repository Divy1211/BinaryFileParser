use crate::types::diff::{Diff, Diffable};

impl<T> Diffable for Vec<T> {
    type DiffResult = Vec<(usize, Diff<T>)>;

    fn diff(&self, other: &Self) -> Self::DiffResult {
        todo!()
    }
}