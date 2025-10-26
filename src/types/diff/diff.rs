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
