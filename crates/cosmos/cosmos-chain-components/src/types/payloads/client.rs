use cgp::prelude::*;
use core::time::Duration;

use ibc_proto::ibc::lightclients::tendermint::v1::Fraction;

use crate::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState, TendermintHeader,
};

#[derive(Debug, Clone)]
pub struct CosmosUpdateClientPayload {
    pub headers: Vec<TendermintHeader>,
}

#[derive(Debug, Clone, HasField)]
pub struct CosmosCreateClientPayload {
    pub client_state: TendermintClientState,
    pub consensus_state: TendermintConsensusState,
}

pub struct CosmosCreateClientOptions {
    pub max_clock_drift: Duration,
    pub trusting_period: Option<Duration>,
    pub trust_threshold: Fraction,
}

#[derive(Debug, Clone, HasField)]
pub struct CosmosCreateClientPayloadV2 {
    pub client_state: ProtoTendermintClientState,
    pub consensus_state: ProtoTendermintConsensusState,
}
