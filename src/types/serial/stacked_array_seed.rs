use std::fmt;

use serde::de::{SeqAccess, Visitor};

use crate::types::bfp_list::BfpList;
use crate::types::bfp_type::BfpType;
use crate::types::le::array::Array;
use crate::types::le::stacked_array::StackedArray;
use crate::types::parseable_type::ParseableType;
use crate::types::serial::array_seed::ArraySeed;
use crate::types::serial::type_deserializer::TypeDeserializer;

pub struct StackedArraySeed<'a, 'b, 'c>(pub TypeDeserializer<'a, 'b>, pub &'c StackedArray);

impl<'de> Visitor<'de> for StackedArraySeed<'_, '_, '_> {
    type Value = BfpList;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a sequence of sequences")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let StackedArraySeed(TypeDeserializer(data_type, ctx), stacked) = self;
        let mut out = Vec::new();
        
        while let Some(inner) = seq.next_element_seed(ArraySeed(TypeDeserializer(data_type, ctx)))? {
            out.push(ParseableType::Array(inner));
        }
        
        Ok(BfpList::new(out, BfpType::Array(Array::from_stacked(stacked))))
    }
}