use cgp::core::Async;
use hermes_cosmos_chain_components::types::status::Time;
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusStateFieldGetter, HasConsensusStateType, ProvideConsensusStateType,
};
use hermes_relayer_components::chain::traits::types::timestamp::HasTimeType;

use crate::types::consensus_state::AnyConsensusState;

pub struct ProvideAnyConsensusState;

impl<Chain, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideAnyConsensusState
where
    Chain: Async,
{
    type ConsensusState = AnyConsensusState;
}

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
