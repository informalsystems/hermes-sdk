use cgp_core::prelude::*;

#[derive_component(SchemaTypeComponent, ProvideSchemaType<Encoding>)]
pub trait HasSchemaType: Async {
    type Schema: Async;
}
