use core::marker::PhantomData;

use crate::types::tendermint::TendermintClientState;
use hermes_relayer_components::encode::traits::schema::{HasSchemaType, SchemaGetter};

pub struct CosmosTypeUrlSchemas;

macro_rules! impl_type_url {
    ($type:ty, $type_url:literal) => {
        impl<Encoding> SchemaGetter<Encoding, $type> for CosmosTypeUrlSchemas
        where
            Encoding: HasSchemaType<Schema = &'static str>,
        {
            fn schema(_encoding: &Encoding, _phantom: PhantomData<$type>) -> &&'static str {
                &$type_url
            }
        }
    };
}

impl_type_url!(
    TendermintClientState,
    "/ibc.lightclients.tendermint.v1.ClientState"
);
