use cgp::prelude::*;
use hermes_cosmos_relayer::presets::chain::ConsensusStateTypeComponent;
use hermes_relayer_components::chain::traits::types::consensus_state::ProvideConsensusStateType;

use crate::types::consensus_state::SolomachineConsensusState;

pub struct ProvideSolomachineConsensusState;

#[cgp_provider(ConsensusStateTypeComponent)]
impl<Chain, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideSolomachineConsensusState
where
    Chain: Async,
{
    type ConsensusState = SolomachineConsensusState;
}
