use hermes_core::relayer_components::chain::traits::{
    ConsensusStateFieldComponent, ConsensusStateFieldGetter, ConsensusStateTypeComponent,
    HasConsensusStateType, HasTimeType, ProvideConsensusStateType, ProvideRawConsensusStateType,
    RawConsensusStateTypeComponent,
};
use hermes_prelude::*;
use prost_types::Any;
use tendermint::Time;

use crate::types::TendermintConsensusState;

pub struct ProvideTendermintConsensusState;

#[cgp_provider(ConsensusStateTypeComponent)]
impl<Chain, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideTendermintConsensusState
where
    Chain: Async,
{
    type ConsensusState = TendermintConsensusState;
}

#[cgp_provider(ConsensusStateFieldComponent)]
impl<Chain, Counterparty> ConsensusStateFieldGetter<Chain, Counterparty>
    for ProvideTendermintConsensusState
where
    Chain: HasConsensusStateType<Counterparty, ConsensusState = TendermintConsensusState>,
    Counterparty: HasTimeType<Time = Time>,
{
    fn consensus_state_timestamp(consensus_state: &TendermintConsensusState) -> Counterparty::Time {
        consensus_state.timestamp
    }
}

pub struct ProvideAnyRawConsensusState;

#[cgp_provider(RawConsensusStateTypeComponent)]
impl<Chain> ProvideRawConsensusStateType<Chain> for ProvideAnyRawConsensusState
where
    Chain: Async,
{
    type RawConsensusState = Any;
}
