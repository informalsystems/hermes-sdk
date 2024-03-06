#[macro_export]
macro_rules! impl_type_url {
    ($component:ident, $type_url:literal) => {
        pub struct $component;

        impl<Encoding, Value> $crate::vendor::SchemaGetter<Encoding, Value> for $component
        where
            Encoding: $crate::vendor::HasSchemaType<Schema = &'static str>,
        {
            fn schema(
                _encoding: &Encoding,
                _phantom: core::marker::PhantomData<Value>,
            ) -> &&'static str {
                &$type_url
            }
        }
    };
}
