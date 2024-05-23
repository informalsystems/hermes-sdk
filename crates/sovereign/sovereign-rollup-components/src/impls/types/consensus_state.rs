use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::consensus_state::ProvideConsensusStateType;

use crate::types::consensus_state::SovereignConsensusState;

pub struct ProvideSovereignConsensusState;

impl<Chain, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideSovereignConsensusState
where
    Chain: Async,
{
    type ConsensusState = SovereignConsensusState;
}
