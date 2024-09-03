use cgp::prelude::*;
use hermes_protobuf_encoding_components::impl_type_url;

use crate::types::tendermint::{TendermintClientState, TendermintConsensusState};

pub struct CosmosTypeUrlSchemas;

delegate_components! {
    CosmosTypeUrlSchemas {
        TendermintClientState: TendermintClientStateUrl,
        TendermintConsensusState: TendermintConsensusStateUrl,
    }
}

impl_type_url!(
    TendermintClientStateUrl,
    "/ibc.lightclients.tendermint.v1.ClientState",
);

impl_type_url!(
    TendermintConsensusStateUrl,
    "/ibc.lightclients.tendermint.v1.ConsensusState",
);
