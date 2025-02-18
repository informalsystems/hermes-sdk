use cgp::prelude::*;

use crate::traits::types::schema::{ProvideSchemaType, SchemaTypeComponent};

pub struct ProvideStringSchema;

#[cgp_provider(SchemaTypeComponent)]
impl<Encoding> ProvideSchemaType<Encoding> for ProvideStringSchema
where
    Encoding: Async,
{
    type Schema = &'static str;
}
