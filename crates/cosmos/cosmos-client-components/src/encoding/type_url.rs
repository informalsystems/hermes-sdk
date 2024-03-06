use crate::types::tendermint::TendermintClientState;
use cgp_core::prelude::*;
use hermes_protobuf_components::impl_type_url;

pub struct CosmosTypeUrlSchemas;

delegate_components! {
    CosmosTypeUrlSchemas {
        TendermintClientState: TendermintClientStateUrl,
    }
}

impl_type_url!(
    TendermintClientStateUrl,
    "/ibc.lightclients.tendermint.v1.ClientState"
);
