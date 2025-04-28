use hermes_chain_type_components::traits::{HasClientIdType, HasConsensusStateType, HasHeightType};
use hermes_prelude::*;

#[cgp_component {
  provider: ConsensusStateQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryConsensusState<Counterparty>:
    HasAsyncErrorType + HasClientIdType<Counterparty>
where
    Counterparty: HasHeightType + HasConsensusStateType<Self>,
{
    async fn query_consensus_state(
        &self,
        client_id: &Self::ClientId,
        height: &Counterparty::Height,
    ) -> Result<Counterparty::ConsensusState, Self::Error>;
}
