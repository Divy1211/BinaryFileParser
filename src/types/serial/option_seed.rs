use std::fmt;

use serde::de::{DeserializeSeed, Visitor};
use serde::Deserializer;

use crate::types::parseable_type::ParseableType;
use crate::types::serial::type_deserializer::TypeDeserializer;

pub struct OptionSeed<'a, 'b>(pub TypeDeserializer<'a, 'b>);

impl<'de> Visitor<'de> for OptionSeed<'_, '_> {
    type Value = Option<ParseableType>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("an optional value")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Some(self.0.deserialize(deserializer)?))
    }
}

