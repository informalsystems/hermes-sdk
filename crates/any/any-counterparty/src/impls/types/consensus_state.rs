use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::consensus_state::ProvideConsensusStateType;

use crate::types::consensus_state::AnyConsensusState;

pub struct ProvideAnyConsensusState;

impl<Chain, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideAnyConsensusState
where
    Chain: Async,
{
    type ConsensusState = AnyConsensusState;
}

// impl<Chain, Counterparty> ProvideConsensusStateF
