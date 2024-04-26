use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ProvideConsensusStateType, ProvideRawConsensusStateType,
};
use prost_types::Any;

use crate::types::tendermint::TendermintConsensusState;

pub struct ProvideTendermintConsensusState;

impl<Chain, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideTendermintConsensusState
where
    Chain: Async,
{
    type ConsensusState = TendermintConsensusState;
}

pub struct ProvideAnyRawConsensusState;

impl<Chain> ProvideRawConsensusStateType<Chain> for ProvideAnyRawConsensusState
where
    Chain: Async,
{
    type RawConsensusState = Any;
}

pub struct ProvideRawConsensusStateBytes;

impl<Chain> ProvideRawConsensusStateType<Chain> for ProvideRawConsensusStateBytes
where
    Chain: Async,
{
    type RawConsensusState = Vec<u8>;
}
