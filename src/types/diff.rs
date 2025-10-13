pub mod struct_diffable;
mod vec_diffable;

#[derive(Debug, Clone)]
pub enum Diff<T> {
    None,
    Inserted(T),
    Deleted(T),
    Changed(T),
    Nested(Vec<(usize, Diff<T>)>)
}

pub trait Diffable<T> : Sized {
    fn diff(&self, other: &Self) -> Diff<T>;
}