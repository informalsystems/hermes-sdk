use hermes_prelude::*;
use hermes_protobuf_encoding_components::impl_type_url;
use ibc::clients::tendermint::types::{
    TENDERMINT_CLIENT_STATE_TYPE_URL, TENDERMINT_CONSENSUS_STATE_TYPE_URL,
};

use crate::types::{TendermintClientState, TendermintConsensusState};

pub struct CosmosTypeUrlSchemas;

impl_type_url!(
    CosmosTypeUrlSchemas,
    TendermintClientState,
    TENDERMINT_CLIENT_STATE_TYPE_URL,
);

impl_type_url!(
    CosmosTypeUrlSchemas,
    TendermintConsensusState,
    TENDERMINT_CONSENSUS_STATE_TYPE_URL,
);
