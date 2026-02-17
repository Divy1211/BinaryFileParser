use pyo3::{pyclass, PyResult};

use crate::combinators::combinator::Combinator;
use crate::combinators::r#if::if_break::IfBreak;
use crate::combinators::r#if::if_check::IfCheck;
use crate::combinators::r#if::if_check_key::IfCheckKey;
use crate::combinators::r#if::if_cmp_by::IfCmpBy;
use crate::combinators::r#if::if_cmp_from::IfCmpFrom;
use crate::combinators::r#if::if_cmp_key::IfCmpKey;
use crate::combinators::r#if::if_cmp_key_to::IfCmpKeyTo;
use crate::combinators::r#if::if_cmp_len_by::IfCmpLenBy;
use crate::combinators::r#if::if_cmp_len_from::IfCmpLenFrom;
use crate::combinators::r#if::if_cmp_len_to::IfCmpLenTo;
use crate::combinators::r#if::if_cmp_to::IfCmpTo;
use crate::combinators::r#if::if_else::IfElse;
use crate::combinators::r#if::if_is_none::IfIsNone;
use crate::combinators::r#if::if_key_is_none::IfKeyIsNone;
use crate::combinators::r#if::if_ver::IfVer;
use crate::combinators::set::set_by::SetBy;
use crate::combinators::set::set_from::SetFrom;
use crate::combinators::set::set_from_key::SetFromKey;
use crate::combinators::set::set_from_len::SetFromLen;
use crate::combinators::set::set_to::SetTo;
use crate::combinators::set_key::set_key_by::SetKeyBy;
use crate::combinators::set_key::set_key_from::SetKeyFrom;
use crate::combinators::set_key::set_key_from_len::SetKeyFromLen;
use crate::combinators::set_key::set_key_to::SetKeyTo;
use crate::combinators::set_repeat::set_repeat_by::SetRepeatBy;
use crate::combinators::set_repeat::set_repeat_from::SetRepeatFrom;
use crate::combinators::set_repeat::set_repeat_from_key::SetRepeatFromKey;
use crate::combinators::set_repeat::set_repeat_from_len::SetRepeatFromLen;
use crate::combinators::set_repeat::set_repeat_to::SetRepeatTo;

use crate::impl_from_for_combinator_type;
use crate::retrievers::retriever::Retriever;
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

#[pyclass(module = "bfp_rs.combinators")]
#[derive(Debug, Clone)]
pub enum CombinatorType {
    SetRepeatFrom(SetRepeatFrom),
    SetRepeatBy(SetRepeatBy),
    SetRepeatFromLen(SetRepeatFromLen),
    SetRepeatFromKey(SetRepeatFromKey),
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

    IfCheckKey(IfCheckKey),
    IfKeyIsNone(IfKeyIsNone),
    IfCmpKey(IfCmpKey),
    IfCmpKeyTo(IfCmpKeyTo),

    IfElse(IfElse),
    IfBreak(IfBreak),

    SetKeyFrom(SetKeyFrom),
    SetKeyTo(SetKeyTo),
    SetKeyBy(SetKeyBy),
    SetKeyFromLen(SetKeyFromLen),

    SetFrom(SetFrom),
    SetFromKey(SetFromKey),
    SetBy(SetBy),
    SetFromLen(SetFromLen),
    SetTo(SetTo),
}

impl Combinator for CombinatorType {
    fn run(
        &self,
        retrievers: &Vec<Retriever>,
        data: &mut Vec<Option<ParseableType>>,
        repeats: &mut Vec<Option<isize>>,
        ver: &Version,
        ctx: &mut Context,
    ) -> PyResult<()> {
        match self {
            CombinatorType::SetRepeatFrom(com)    => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::SetRepeatFromKey(com) => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::SetRepeatBy(com)      => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::SetRepeatFromLen(com) => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::SetRepeatTo(com)      => com.run(retrievers, data, repeats, ver, ctx),

            CombinatorType::IfCmpBy(com)          => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::IfCmpFrom(com)        => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::IfCmpTo(com)          => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::IfCmpLenBy(com)       => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::IfCmpLenFrom(com)     => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::IfCmpLenTo(com)       => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::IfCheck(com)          => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::IfIsNone(com)         => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::IfVer(com)            => com.run(retrievers, data, repeats, ver, ctx),

            CombinatorType::IfCheckKey(com)       => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::IfKeyIsNone(com)      => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::IfCmpKey(com)         => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::IfCmpKeyTo(com)       => com.run(retrievers, data, repeats, ver, ctx),

            CombinatorType::IfElse(com)           => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::IfBreak(com)          => com.run(retrievers, data, repeats, ver, ctx),
            
            CombinatorType::SetKeyFrom(com)       => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::SetKeyBy(com)         => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::SetKeyFromLen(com)    => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::SetKeyTo(com)         => com.run(retrievers, data, repeats, ver, ctx),

            CombinatorType::SetFrom(com)          => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::SetFromKey(com)       => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::SetBy(com)            => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::SetFromLen(com)       => com.run(retrievers, data, repeats, ver, ctx),
            CombinatorType::SetTo(com)            => com.run(retrievers, data, repeats, ver, ctx),
        }
    }
}

impl CombinatorType {
    pub fn uses_keys(&self) -> bool {
        match self {
            CombinatorType::SetRepeatFromKey(_) => true,
            
            CombinatorType::IfCheckKey(_)       => true,
            CombinatorType::IfKeyIsNone(_)      => true,
            CombinatorType::IfCmpKey(_)         => true,
            CombinatorType::IfCmpKeyTo(_)       => true,

            CombinatorType::SetKeyFrom(_)       => true,
            CombinatorType::SetKeyTo(_)         => true,
            CombinatorType::SetKeyBy(_)         => true,
            CombinatorType::SetKeyFromLen(_)    => true,
            CombinatorType::SetFromKey(_)       => true,
            
            _ => false,
        }
    }
}

impl_from_for_combinator_type!(SetRepeatFrom, SetRepeatFrom);
impl_from_for_combinator_type!(SetRepeatBy, SetRepeatBy);
impl_from_for_combinator_type!(SetRepeatFromLen, SetRepeatFromLen);
impl_from_for_combinator_type!(SetRepeatFromKey, SetRepeatFromKey);
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

impl_from_for_combinator_type!(IfCheckKey, IfCheckKey);
impl_from_for_combinator_type!(IfKeyIsNone, IfKeyIsNone);
impl_from_for_combinator_type!(IfCmpKey, IfCmpKey);
impl_from_for_combinator_type!(IfCmpKeyTo, IfCmpKeyTo);

impl_from_for_combinator_type!(IfElse, IfElse);
impl_from_for_combinator_type!(IfBreak, IfBreak);

impl_from_for_combinator_type!(SetKeyFrom, SetKeyFrom);
impl_from_for_combinator_type!(SetKeyTo, SetKeyTo);
impl_from_for_combinator_type!(SetKeyBy, SetKeyBy);
impl_from_for_combinator_type!(SetKeyFromLen, SetKeyFromLen);

impl_from_for_combinator_type!(SetFrom, SetFrom);
impl_from_for_combinator_type!(SetFromKey, SetFromKey);
impl_from_for_combinator_type!(SetBy, SetBy);
impl_from_for_combinator_type!(SetFromLen, SetFromLen);
impl_from_for_combinator_type!(SetTo, SetTo);
