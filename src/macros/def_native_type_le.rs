#[macro_export]
macro_rules! def_num_type_le {
    ($name:ident, $py_name:expr, $native_type:ty, $size:expr) => {
        #[pyclass(module = "bfp_rs.types.le", name = $py_name)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name;

        impl Parseable for $name {
            type Type = $native_type;

            #[cfg_attr(feature = "inline_always", inline(always))]
            fn from_stream(&self, stream: &mut ByteStream, _ver: &Version) -> PyResult<Self::Type> {
                let bytes = stream.get($size)?.try_into().expect("infallible");
                Ok(Self::Type::from_le_bytes(bytes))
            }

            #[cfg_attr(feature = "inline_always", inline(always))]
            fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()> {
                buffer.extend_from_slice(&value.to_le_bytes());
                Ok(())
            }
        }

        wrap_py!($name);
    };
}

#[macro_export]
macro_rules! def_bool_type_le {
    ($name:ident, $py_name:expr, $native_type:ty, $size:expr) => {
        #[pyclass(module = "bfp_rs.types.le", name = $py_name)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name;
        
        impl Parseable for $name {
            type Type = bool;
        
            #[cfg_attr(feature = "inline_always", inline(always))]
            fn from_stream(&self, stream: &mut ByteStream, _ver: &Version) -> PyResult<Self::Type> {
                let bytes = stream.get($size)?.try_into().expect("infallible");
                Ok(<$native_type>::from_le_bytes(bytes) != 0)
            }
        
            #[cfg_attr(feature = "inline_always", inline(always))]
            fn to_bytes_in(&self, value: &Self::Type, buffer: &mut Vec<u8>) -> PyResult<()> {
                buffer.extend_from_slice(&<$native_type>::to_le_bytes(if *value { 1 } else { 0 }));
                Ok(())
            }
        }
        
        wrap_py!($name);
    };
}