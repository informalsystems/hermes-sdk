use core::marker::PhantomData;

use hermes_prelude::*;

use crate::traits::HasSchemaType;

#[cgp_component {
  provider: SchemaGetter,
  context: Encoding,
}]
pub trait HasSchema<Value>: HasSchemaType {
    fn schema(&self, phantom: PhantomData<Value>) -> &Self::Schema;
}
