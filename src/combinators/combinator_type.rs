use pyo3::{pyclass, PyResult};

use crate::combinators::combinator::Combinator;
use crate::combinators::r#if::if_check::IfCheck;
use crate::combinators::r#if::if_cmp_by::IfCmpBy;
use crate::combinators::r#if::if_cmp_from::IfCmpFrom;
use crate::combinators::r#if::if_cmp_len_by::IfCmpLenBy;
use crate::combinators::r#if::if_cmp_len_from::IfCmpLenFrom;
use crate::combinators::r#if::if_cmp_len_to::IfCmpLenTo;
use crate::combinators::r#if::if_cmp_to::IfCmpTo;
use crate::combinators::r#if::if_is_none::IfIsNone;
use crate::combinators::r#if::if_ver::IfVer;
use crate::combinators::set::set_by::SetBy;
use crate::combinators::set::set_from::SetFrom;
use crate::combinators::set::set_from_len::SetFromLen;
use crate::combinators::set::set_to::SetTo;
use crate::combinators::set_repeat::set_repeat_by::SetRepeatBy;
use crate::combinators::set_repeat::set_repeat_from::SetRepeatFrom;
use crate::combinators::set_repeat::set_repeat_from_len::SetRepeatFromLen;
use crate::combinators::set_repeat::set_repeat_to::SetRepeatTo;

use crate::impl_from_for_combinator_type;
use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub enum CombinatorType {
    SetRepeatFrom(SetRepeatFrom),
    SetRepeatBy(SetRepeatBy),
    SetRepeatFromLen(SetRepeatFromLen),
    SetRepeatTo(SetRepeatTo),

    IfCmpBy(IfCmpBy),
    IfCmpFrom(IfCmpFrom),
    IfCmpTo(IfCmpTo),
    IfCheck(IfCheck),
    IfIsNone(IfIsNone),
    IfCmpLenBy(IfCmpLenBy),
    IfCmpLenFrom(IfCmpLenFrom),
    IfCmpLenTo(IfCmpLenTo),
    IfVer(IfVer),

    SetFrom(SetFrom),
    SetBy(SetBy),
    SetFromLen(SetFromLen),
    SetTo(SetTo),
}

impl Combinator for CombinatorType {
    fn run(&self, retrievers: &Vec<Retriever>, data: &mut Vec<Option<ParseableType>>, repeats: &mut Vec<Option<isize>>, ver: &Version) -> PyResult<()> {
        match self {
            CombinatorType::SetRepeatFrom(com)    => com.run(retrievers, data, repeats, ver),
            CombinatorType::SetRepeatBy(com)      => com.run(retrievers, data, repeats, ver),
            CombinatorType::SetRepeatFromLen(com) => com.run(retrievers, data, repeats, ver),
            CombinatorType::SetRepeatTo(com)      => com.run(retrievers, data, repeats, ver),

            CombinatorType::IfCmpBy(com)          => com.run(retrievers, data, repeats, ver),
            CombinatorType::IfCmpFrom(com)        => com.run(retrievers, data, repeats, ver),
            CombinatorType::IfCmpTo(com)          => com.run(retrievers, data, repeats, ver),
            CombinatorType::IfCmpLenBy(com)       => com.run(retrievers, data, repeats, ver),
            CombinatorType::IfCmpLenFrom(com)     => com.run(retrievers, data, repeats, ver),
            CombinatorType::IfCmpLenTo(com)       => com.run(retrievers, data, repeats, ver),
            CombinatorType::IfCheck(com)          => com.run(retrievers, data, repeats, ver),
            CombinatorType::IfIsNone(com)         => com.run(retrievers, data, repeats, ver),
            CombinatorType::IfVer(com)            => com.run(retrievers, data, repeats, ver),
            
            CombinatorType::SetFrom(com)          => com.run(retrievers, data, repeats, ver),
            CombinatorType::SetBy(com)            => com.run(retrievers, data, repeats, ver),
            CombinatorType::SetFromLen(com)       => com.run(retrievers, data, repeats, ver),
            CombinatorType::SetTo(com)            => com.run(retrievers, data, repeats, ver),
        }
    }
}

impl_from_for_combinator_type!(SetRepeatFrom, SetRepeatFrom);
impl_from_for_combinator_type!(SetRepeatBy, SetRepeatBy);
impl_from_for_combinator_type!(SetRepeatFromLen, SetRepeatFromLen);
impl_from_for_combinator_type!(SetRepeatTo, SetRepeatTo);

impl_from_for_combinator_type!(IfCmpFrom, IfCmpFrom);
impl_from_for_combinator_type!(IfCmpBy, IfCmpBy);
impl_from_for_combinator_type!(IfCmpTo, IfCmpTo);
impl_from_for_combinator_type!(IfCmpLenBy, IfCmpLenBy);
impl_from_for_combinator_type!(IfCmpLenFrom, IfCmpLenFrom);
impl_from_for_combinator_type!(IfCmpLenTo, IfCmpLenTo);
impl_from_for_combinator_type!(IfCheck, IfCheck);
impl_from_for_combinator_type!(IfIsNone, IfIsNone);
impl_from_for_combinator_type!(IfVer, IfVer);

impl_from_for_combinator_type!(SetFrom, SetFrom);
impl_from_for_combinator_type!(SetBy, SetBy);
impl_from_for_combinator_type!(SetFromLen, SetFromLen);
impl_from_for_combinator_type!(SetTo, SetTo);
