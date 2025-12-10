#[macro_export]
macro_rules! impl_into_pyobj {
    ($variant:ident, $base:ident) => {
        impl $variant {
            pub fn into_pyany(self, py: Python) -> Py<PyAny> {
                Py::new(
                    py,
                    PyClassInitializer::from($base).add_subclass(self)
                ).expect("infallible").into_any()
            }
        }
    };
}

#[macro_export]
macro_rules! match_args_type {
    () => { () };
    ($field:ident) => { (&'static str,) };
    ($field1:ident, $field2:ident) => { (&'static str, &'static str) };
    ($field1:ident, $field2:ident, $field3:ident) => { (&'static str, &'static str, &'static str) };
}

#[macro_export]
macro_rules! make_struct {
    ($name:ident($base:ident) as $py_name:literal { $($field:ident: $ty:ty),* $(,)? } impl { $($extra:tt)* } ) => {
        #[pyclass(name = $py_name, extends = $base)]
        pub struct $name {
            $(
                #[pyo3(get)]
                pub $field: $ty,
            )*
        }

        #[pymethods]
        impl $name {
            #[classattr]
            #[allow(non_upper_case_globals)]
            const __match_args__: match_args_type!($($field),*) = ($(stringify!($field),)*);

           $($extra)*
        }

        impl_into_pyobj!($name, $base);
    };
}