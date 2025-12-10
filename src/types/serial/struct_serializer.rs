use serde::{Serialize, Serializer};
use serde::ser::{SerializeMap};
use crate::retrievers::retriever::RetState;
use crate::types::base_struct::BaseStruct;
use crate::types::parseable_type::ParseableType;
use crate::types::r#struct::Struct;

pub struct StructSerializer<'a, 'b>(pub &'a Struct, pub &'b BaseStruct);

impl Serialize for StructSerializer<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let struct_ = self.0;
        let value = self.1;
        let mut inner = value.inner_mut();

        let retrievers = &struct_.raw.retrievers;

        let mut s = serializer.serialize_map(None)?;

        s.serialize_entry("ver", &inner.ver)?;
        for retriever in retrievers.iter() {
            if !retriever.supported(&inner.ver) {
                continue;
            }

            let (data, repeats, ver) = inner.split();

            retriever.call_on_writes(retrievers, data, repeats, ver).map_err(|py_err| {
                serde::ser::Error::custom(format!(
                    "Python error during serialization: {}",
                    py_err
                ))
            })?;

            let value = inner.data[retriever.idx].as_ref().expect("supported check done above");

            match retriever.state(&inner.repeats) {
                RetState::NoneList | RetState::NoneValue => { s.serialize_entry(&retriever.name, &Option::<i32>::None)?; },
                RetState::Value => {
                    s.serialize_entry(&retriever.name, value)?;
                }
                RetState::List => {
                    let ParseableType::Array(ls) = value else {
                        unreachable!("Retriever state guarantee broken while writing '{}'", retriever.name)
                    };
                    s.serialize_entry(&retriever.name, ls)?;
                }
            }
        }
        s.end()
    }
}