use serde::de::DeserializeSeed;
use serde::{Deserialize, Deserializer};

use crate::types::bfp_list::BfpList;
use crate::types::bfp_type::BfpType;
use crate::types::context::Context;
use crate::types::le::array::Array;
use crate::types::le::option::OptionType;
use crate::types::le::stacked_array::StackedArray;
use crate::types::le::stacked_attr_array::StackedAttrArray;
use crate::types::le::str::Str;
use crate::types::le::tail::Tail;
use crate::types::parseable_type::ParseableType;
use crate::types::serial::array_seed::ArraySeed;
use crate::types::serial::option_seed::OptionSeed;
use crate::types::serial::stacked_array_seed::StackedArraySeed;
use crate::types::serial::struct_deserializer::StructDeserializer;

pub struct TypeDeserializer<'a, 'b>(pub &'a BfpType, pub &'b mut Context);

impl<'de> DeserializeSeed<'de> for TypeDeserializer<'_, '_> {
    type Value = ParseableType;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match &self.0 {
            BfpType::UInt8(_) => Ok(ParseableType::UInt8(u8::deserialize(deserializer)?)),
            BfpType::UInt16(_) => Ok(ParseableType::UInt16(u16::deserialize(deserializer)?)),
            BfpType::UInt32(_) => Ok(ParseableType::UInt32(u32::deserialize(deserializer)?)),
            BfpType::UInt64(_) => Ok(ParseableType::UInt64(u64::deserialize(deserializer)?)),
            BfpType::UInt128(_) => Ok(ParseableType::UInt128(u128::deserialize(deserializer)?)),
            
            BfpType::Int8(_) => Ok(ParseableType::Int8(i8::deserialize(deserializer)?)),
            BfpType::Int16(_) => Ok(ParseableType::Int16(i16::deserialize(deserializer)?)),
            BfpType::Int32(_) => Ok(ParseableType::Int32(i32::deserialize(deserializer)?)),
            BfpType::Int64(_) => Ok(ParseableType::Int64(i64::deserialize(deserializer)?)),
            BfpType::Int128(_) => Ok(ParseableType::Int128(i128::deserialize(deserializer)?)),
            
            BfpType::Float32(_) => Ok(ParseableType::Float32(f32::deserialize(deserializer)?)),
            BfpType::Float64(_) => Ok(ParseableType::Float64(f64::deserialize(deserializer)?)),
            
            BfpType::Bool8(_)
            | BfpType::Bool16(_)
            | BfpType::Bool32(_)
            | BfpType::Bool64(_)
            | BfpType::Bool128(_) => {
                Ok(ParseableType::Bool(bool::deserialize(deserializer)?))
            }
            
            BfpType::Str(_) | BfpType::NTStr(_) => {
                Ok(ParseableType::Str(String::deserialize(deserializer)?))
            }

            BfpType::StrArray(arr) => {
                let ls = Vec::<String>::deserialize(deserializer)?
                    .into_iter()
                    .map(ParseableType::Str)
                    .collect();
                let ty = BfpType::Str(Str::from_arr(arr));
                Ok(ParseableType::Array(BfpList::new(ls, ty)))
            }
            
            BfpType::Bytes(_) => Ok(ParseableType::Bytes(Vec::<u8>::deserialize(deserializer)?)),
            
            BfpType::Option(OptionType { data_type, .. }) => {
                Ok(ParseableType::Option(
                    deserializer.deserialize_option(OptionSeed(TypeDeserializer(data_type.as_ref(), self.1)))?
                        .map(Box::new))
                )
            }

            BfpType::Array(Array { data_type, .. })
            | BfpType::StackedAttrArray(StackedAttrArray { data_type, .. })
            | BfpType::Tail(Tail { data_type, .. }) => {
                Ok(ParseableType::Array(
                    deserializer.deserialize_seq(ArraySeed(TypeDeserializer(data_type.as_ref(), self.1)))?
                ))
            }

            BfpType::StackedArray(arr @ StackedArray { data_type, .. }) => {
                Ok(ParseableType::Array(
                    deserializer.deserialize_seq(StackedArraySeed(TypeDeserializer(data_type.as_ref(), self.1), arr))?
                ))
            },
            
            BfpType::Struct(struct_) => {
                let base = StructDeserializer(struct_, self.1).deserialize(deserializer)?;
                Ok(ParseableType::Struct {
                    val: base,
                    struct_: struct_.clone(),
                })
            }
        }
    }
}