use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::consensus_state::ProvideConsensusStateType;

use crate::types::consensus_state::SolomachineConsensusState;

pub struct ProvideSolomachineConsensusState;

impl<Chain, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideSolomachineConsensusState
where
    Chain: Async,
{
    type ConsensusState = SolomachineConsensusState;
}
