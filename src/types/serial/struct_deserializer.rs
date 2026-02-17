use std::fmt;
use std::collections::HashMap;

use serde::de::{DeserializeSeed, Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use crate::retrievers::retriever::RetState;
use crate::types::base_struct::BaseStruct;
use crate::types::serial::array_seed::ArraySeed;
use crate::types::context::Context;
use crate::types::parseable_type::ParseableType;
use crate::types::r#struct::Struct;
use crate::types::serial::type_deserializer::TypeDeserializer;
use crate::types::version::Version;

pub struct StructDeserializer<'a, 'b>(pub &'a Struct, pub &'b mut Context);

impl<'de> DeserializeSeed<'de> for StructDeserializer<'_, '_> {
    type Value = BaseStruct;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de> Visitor<'de> for StructDeserializer<'_, '_> {
    type Value = BaseStruct;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a map matching retriever names")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let retrievers = &self.0.raw.retrievers;
        let ctx = self.1;
        let mut data = Vec::with_capacity(retrievers.len());
        let mut repeats = vec![None; retrievers.len()];

        let mut values = HashMap::with_capacity(retrievers.len() + 1);
        while let Some((key, value)) = access.next_entry::<String, serde_json::Value>()? {
            values.insert(key, value);
        }
        
        let Some(ver) = values.get("ver") else {
            return Err(Error::custom("Invalid Object: Version not found"));
        };
        let ver = Version::deserialize(ver).map_err(Error::custom)?;
        
        for (i, retriever) in retrievers.iter().enumerate() {
            if !retriever.supported(&ver) {
                data.push(None);
                continue;
            }
            
            let Some(value) = values.remove(&retriever.name) else {
                return Err(Error::custom(format!("Invalid Object: '{}' not found", retriever.name)));
            };

            data.push(Some(match retriever.state(&repeats) {
                RetState::Value | RetState::NoneValue if value.is_null() => {
                    repeats[i] = Some(-1);
                    ParseableType::None
                },
                RetState::List | RetState::NoneList if value.is_null() => {
                    repeats[i] = Some(-2);
                    ParseableType::None
                },
                RetState::Value | RetState::NoneValue => {
                    repeats[i] = None;
                    TypeDeserializer(&retriever.data_type, ctx)
                        .deserialize(value)
                        .map_err(|e| Error::custom(format!("Error occurred while reading '{}': {e}", retriever.name)))?
                }
                RetState::List | RetState::NoneList => {
                    let repeat = retriever.repeat(&repeats);
                    if !value.is_array() {
                        return Err(Error::custom(format!(
                            "Invalid Object: {} should be an array", retriever.name
                        )))
                    }
                    let len = value.as_array().expect("Infallible").len() as isize;
                    if repeat == -2 {
                        repeats[i] = Some(len);
                    } else if repeats[i].is_none() && repeat != len {
                        return Err(Error::custom(format!(
                            "List length mismatch for '{}' which is a retriever of fixed repeat. Expected: {repeat}, Actual: {len}", retriever.name
                        )))
                    }
                    let ls = ArraySeed(TypeDeserializer(&retriever.data_type, ctx))
                        .deserialize(value)
                        .map_err(|e| Error::custom(format!("Error occurred while reading '{}': {e}", retriever.name)))?;
                    ParseableType::Array(ls)
                }
            }));
            retriever.call_on_reads(retrievers, &mut data, &mut repeats, &ver, ctx).map_err(|e| {
                Error::custom(e)
            })?;
        }
        Ok(BaseStruct::new(ver, data, repeats))
    }
}