#[macro_export]
macro_rules! impl_from_for_parseable_type {
    ($type_:ty, $variant:ident) => {
        impl From<$type_> for ParseableType {
            fn from(value: $type_) -> Self {
                ParseableType::$variant(value)
            }
        }
        
        impl TryFrom<&ParseableType> for $type_ {
            type Error = ();
        
            fn try_from(value: &ParseableType) -> Result<Self, Self::Error> {
                match value {
                    ParseableType::$variant(val) => Ok(val.clone()),
                    _ => Err(()),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_into_for_parseable_type {
    ($type_:ty) => {
        impl TryFrom<&ParseableType> for $type_ {
            type Error = ();
        
            fn try_from(value: &ParseableType) -> Result<Self, Self::Error> {
                match value {
                    ParseableType::Int8(val)   => Ok(*val as $type_),
                    ParseableType::Int16(val)  => Ok(*val as $type_),
                    ParseableType::Int32(val)  => Ok(*val as $type_),
                    ParseableType::Int64(val)  => Ok(*val as $type_),
                    ParseableType::Int128(val) => Ok(*val as $type_),
                    
                    ParseableType::UInt8(val)   => Ok(*val as $type_),
                    ParseableType::UInt16(val)  => Ok(*val as $type_),
                    ParseableType::UInt32(val)  => Ok(*val as $type_),
                    ParseableType::UInt64(val)  => Ok(*val as $type_),
                    ParseableType::UInt128(val) => Ok(*val as $type_),

                    ParseableType::Bool(val)    => Ok(*val as $type_),
                    
                    _ => Err(()),
                }
            }
        }
    };
}