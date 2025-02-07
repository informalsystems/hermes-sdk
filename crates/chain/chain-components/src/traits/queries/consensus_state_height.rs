use alloc::vec::Vec;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::types::height::HasHeightType;

#[cgp_component {
  provider: ConsensusStateHeightQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryConsensusStateHeight<Counterparty>:
    HasClientIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasHeightType,
{
    /**
       Query the chain to find a consensus state that has a height that is
       less than or equal the target height. This is needed as a base trusted
       height to build the headers for UpdateClient.

       Invariant: the returned height must be less than or equal to the given
       target height.
    */
    async fn find_consensus_state_height_before(
        &self,
        client_id: &Self::ClientId,
        target_height: &Counterparty::Height,
    ) -> Result<Counterparty::Height, Self::Error>;
}

#[cgp_component {
  provider: ConsensusStateHeightsQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryConsensusStateHeights<Counterparty>:
    HasClientIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasHeightType,
{
    async fn query_consensus_state_heights(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<Vec<Counterparty::Height>, Self::Error>;
}

#[cgp_provider(ConsensusStateHeightsQuerierComponent)]
impl<Chain, Counterparty, Components, Delegate> ConsensusStateHeightsQuerier<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasClientIdType<Counterparty> + HasAsyncErrorType,
    Counterparty: HasHeightType,
    Delegate: ConsensusStateHeightsQuerier<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn query_consensus_state_heights(
        chain: &Chain,
        client_id: &Chain::ClientId,
    ) -> Result<Vec<Counterparty::Height>, Chain::Error> {
        Delegate::query_consensus_state_heights(chain, client_id).await
    }
}
