use crate::traits::{HasClientStateType, HasConsensusStateType};

pub struct CreateClientPayload<Chain, Counterparty>
where
    Chain: HasClientStateType<Counterparty> + HasConsensusStateType<Counterparty>,
{
    pub client_state: Chain::ClientState,
    pub consensus_state: Chain::ConsensusState,
}
