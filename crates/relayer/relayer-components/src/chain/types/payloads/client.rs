use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::consensus_state::HasConsensusStateType;

pub struct CreateClientPayload<Chain, Counterparty>
where
    Chain: HasClientStateType<Counterparty> + HasConsensusStateType<Counterparty>,
{
    pub client_state: Chain::ClientState,
    pub consensus_state: Chain::ConsensusState,
}
