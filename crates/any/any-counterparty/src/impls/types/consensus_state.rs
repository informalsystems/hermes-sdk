use cgp::core::Async;
use cgp::prelude::*;
use hermes_cosmos_chain_components::types::Time;
use hermes_relayer_components::chain::traits::{
    ConsensusStateFieldComponent, ConsensusStateFieldGetter, ConsensusStateTypeComponent,
    HasConsensusStateType, HasTimeType, ProvideConsensusStateType,
};

use crate::types::consensus_state::AnyConsensusState;

pub struct ProvideAnyConsensusState;

#[cgp_provider(ConsensusStateTypeComponent)]
impl<Chain, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideAnyConsensusState
where
    Chain: Async,
{
    type ConsensusState = AnyConsensusState;
}

#[cgp_provider(ConsensusStateFieldComponent)]
impl<Chain, Counterparty> ConsensusStateFieldGetter<Chain, Counterparty>
    for ProvideAnyConsensusState
where
    Chain: HasConsensusStateType<Counterparty, ConsensusState = AnyConsensusState>,
    Counterparty: HasTimeType<Time = Time>,
{
    fn consensus_state_timestamp(consensus_state: &AnyConsensusState) -> Counterparty::Time {
        match consensus_state {
            AnyConsensusState::Tendermint(consensus_state) => consensus_state.timestamp,
        }
    }
}
