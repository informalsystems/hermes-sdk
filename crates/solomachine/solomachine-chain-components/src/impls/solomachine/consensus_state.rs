use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusStateTypeComponent, ProvideConsensusStateType,
};

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
