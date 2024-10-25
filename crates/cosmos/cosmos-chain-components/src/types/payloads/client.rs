use cgp::prelude::*;

use crate::types::tendermint::{TendermintClientState, TendermintConsensusState, TendermintHeader};

#[derive(Debug, Clone)]
pub struct CosmosUpdateClientPayload {
    pub headers: Vec<TendermintHeader>,
}

#[derive(Debug, Clone, HasField)]
pub struct CosmosCreateClientPayload {
    pub client_state: TendermintClientState,
    pub consensus_state: TendermintConsensusState,
}
