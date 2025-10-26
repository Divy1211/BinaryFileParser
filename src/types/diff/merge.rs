use crate::types::diff::diff::{Diff, Diffable, IDiff};

pub trait Mergeable<T>: Diffable<T> {
    fn patch(&mut self, change: IDiff<T>, off: isize) -> isize;
    fn merge(&mut self, slf: &Self, other: &Self) -> Vec<Conflict<T>>;
}

#[derive(Debug, Clone)]
pub enum Conflict<T> {
    Basic(usize, Diff<T>, Diff<T>),
    Nested(usize, Vec<Conflict<T>>)
}