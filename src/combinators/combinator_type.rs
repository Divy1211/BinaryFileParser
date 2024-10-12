use pyo3::{pyclass, PyResult};

use crate::combinators::combinator::Combinator;
use crate::combinators::if_::if_cmp_from::IfCmpFrom;
use crate::combinators::if_::if_cmp_to::IfCmpTo;
use crate::combinators::set_repeat::set_repeat_from::SetRepeatFrom;
use crate::combinators::set_repeat::set_repeat_to::SetRepeatTo;
use crate::impl_from_for_combinator_type;
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass]
#[derive(Debug, Clone)]
pub enum CombinatorType {
    SetRepeatFrom(SetRepeatFrom),
    SetRepeatTo(SetRepeatTo),
    IfCmpFrom(IfCmpFrom),
    IfCmpTo(IfCmpTo),
}

impl Combinator for CombinatorType {
    fn run(&self, retrievers: &Vec<Retriever>, data: &mut Vec<Option<ParseableType>>, repeats: &mut Vec<Option<isize>>, ver: &Version) -> PyResult<()> {
        match self {
            CombinatorType::SetRepeatFrom(com) => com.run(retrievers, data, repeats, ver),
            CombinatorType::SetRepeatTo(com)   => com.run(retrievers, data, repeats, ver),
            CombinatorType::IfCmpFrom(com)     => com.run(retrievers, data, repeats, ver),
            CombinatorType::IfCmpTo(com)       => com.run(retrievers, data, repeats, ver),
        }
    }
}

impl_from_for_combinator_type!(SetRepeatFrom, SetRepeatFrom);
impl_from_for_combinator_type!(SetRepeatTo, SetRepeatTo);
impl_from_for_combinator_type!(IfCmpFrom, IfCmpFrom);
impl_from_for_combinator_type!(IfCmpTo, IfCmpTo);
