use cgp_core::prelude::*;
use hermes_protobuf_encoding_components::impl_type_url;

use crate::types::tendermint::TendermintClientState;

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
