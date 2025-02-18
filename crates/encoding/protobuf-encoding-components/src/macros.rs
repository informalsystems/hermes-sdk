#[macro_export]
macro_rules! impl_type_url {
    ($component:ident, $type:ty, $type_url:tt $(,)?) => {
        #[cgp_provider($crate::vendor::SchemaGetterComponent)]
        impl<Encoding> $crate::vendor::SchemaGetter<Encoding, $type> for $component
        where
            Encoding: $crate::vendor::HasSchemaType<Schema = &'static str>,
        {
            fn schema(
                _encoding: &Encoding,
                _phantom: core::marker::PhantomData<$type>,
            ) -> &&'static str {
                &$type_url
            }
        }
    };
}
