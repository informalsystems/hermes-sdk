use hermes_prelude::*;

#[cgp_component {
  name: SchemaTypeComponent,
  provider: ProvideSchemaType,
  context: Encoding,
}]
pub trait HasSchemaType: Async {
    type Schema: Async;
}
