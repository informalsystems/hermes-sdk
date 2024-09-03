use core::marker::PhantomData;

use cgp::prelude::*;

use crate::traits::types::schema::HasSchemaType;

#[derive_component(SchemaGetterComponent, SchemaGetter<Encoding>)]
pub trait HasSchema<Value>: HasSchemaType {
    fn schema(&self, phantom: PhantomData<Value>) -> &Self::Schema;
}
