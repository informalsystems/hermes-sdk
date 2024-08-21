use cgp_core::prelude::Async;

use crate::traits::types::schema::ProvideSchemaType;

pub struct ProvideStringSchema;

impl<Encoding> ProvideSchemaType<Encoding> for ProvideStringSchema
where
    Encoding: Async,
{
    type Schema = &'static str;
}
