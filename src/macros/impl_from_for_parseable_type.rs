#[macro_export]
macro_rules! impl_from_for_parseable_type {
    ($type_:ty, $variant:ident) => {
        impl From<$type_> for ParseableType {
            fn from(value: $type_) -> Self {
                ParseableType::$variant(value)
            }
        }
        
        impl TryFrom<ParseableType> for $type_ {
            type Error = ();
        
            fn try_from(value: ParseableType) -> Result<Self, Self::Error> {
                match value {
                    ParseableType::$variant(val) => Ok(val),
                    _ => Err(()),
                }
            }
        }
    };
}