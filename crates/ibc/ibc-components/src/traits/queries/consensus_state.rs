use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;
use hermes_chain_type_components::traits::types::ibc::consensus_state::HasConsensusStateType;

#[cgp_component {
  provider: ConsensusStateQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryConsensusState<Counterparty>:
    HasErrorType + HasClientIdType<Counterparty>
where
    Counterparty: HasHeightType + HasConsensusStateType<Self>,
{
    async fn query_consensus_state(
        &self,
        client_id: &Self::ClientId,
        height: &Counterparty::Height,
    ) -> Result<Counterparty::ConsensusState, Self::Error>;
}
