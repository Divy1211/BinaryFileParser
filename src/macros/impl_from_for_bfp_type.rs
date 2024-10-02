#[macro_export]
macro_rules! impl_from_for_bfp_type {
    ($type_:ty, $variant:ident) => {
        impl From<$type_> for BfpType {
            fn from(value: $type_) -> Self {
                BfpType::$variant(value)
            }
        }
    };
}
