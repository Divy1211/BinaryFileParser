pub mod struct_diffable;
mod vec_diffable;

#[derive(Debug, Clone)]
pub enum Diff<T> {
    Inserted(T),
    Deleted(T),
    Changed(T),
    Nested(Vec<(usize, Diff<T>)>)
}

pub trait Diffable : Sized {
    type DiffResult;
    fn diff(&self, other: &Self) -> Self::DiffResult;
}