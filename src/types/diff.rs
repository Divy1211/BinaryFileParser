pub mod struct_diffable;
mod vec_diffable;

#[allow(clippy::module_inception)]
pub mod diff;

pub mod merge;
pub mod struct_mergeable;
pub mod diff_py;
mod macros;
pub mod merge_py;