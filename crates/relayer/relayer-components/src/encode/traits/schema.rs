use core::marker::PhantomData;

use cgp_core::prelude::*;

#[derive_component(SchemaTypeComponent, ProvideSchemaType<Encoding>)]
pub trait HasSchemaType: Async {
    type Schema: Async;
}

#[derive_component(SchemaGetterComponent, SchemaGetter<Encoding>)]
pub trait HasSchema<Value>: HasSchemaType {
    fn schema(&self, phantom: PhantomData<Value>) -> &Self::Schema;
}
