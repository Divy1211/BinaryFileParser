use std::fmt;

use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::Deserializer;

use crate::types::bfp_list::BfpList;
use crate::types::serial::type_deserializer::TypeDeserializer;

pub struct ArraySeed<'a, 'b>(pub TypeDeserializer<'a, 'b>);

impl<'de> DeserializeSeed<'de> for ArraySeed<'_, '_> {
    type Value = BfpList;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(self)
    }
}

impl<'de> Visitor<'de> for ArraySeed<'_, '_> {
    type Value = BfpList;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let ArraySeed(TypeDeserializer(data_type, ctx)) = self;
        let mut out = Vec::new();

        while let Some(elem) = seq.next_element_seed(TypeDeserializer(data_type, ctx))? {
            out.push(elem);
        }
        Ok(BfpList::new(out, self.0.0.clone()))
    }
}