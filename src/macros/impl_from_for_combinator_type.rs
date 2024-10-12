#[macro_export]
macro_rules! impl_from_for_combinator_type {
    ($type_:ty, $variant:ident) => {
        impl From<$type_> for CombinatorType {
            fn from(value: $type_) -> Self {
                CombinatorType::$variant(value)
            }
        }
        
        impl TryFrom<CombinatorType> for $type_ {
            type Error = ();
        
            fn try_from(value: CombinatorType) -> Result<Self, Self::Error> {
                match value {
                    CombinatorType::$variant(val) => Ok(val),
                    _ => Err(()),
                }
            }
        }
    };
}