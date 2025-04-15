use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::HasHeightType;

use crate::traits::{
    CanQueryClientStateWithLatestHeight, ConsensusStateHeightsQuerier,
    ConsensusStateHeightsQuerierComponent, HasClientStateFields,
};

/**
   This is a placeholder implementation for querying all consensus state heights,
   by returning only the latest client state height to be used as the trusted height.

   This would not work in the corner case when the relayer tries to perform
   UpdateClient with a target height that is lower than the latest height.
   In that case, if this is used together with `QueryConsensusStateHeightsAndFindHeightBefore`,
   then the error `NoConsensusStateAtLessThanHeight` would be returned.
   With retry logic, the relayer should be able to retry relaying the packet,
   and use an updated target height that should be higher than the latest height.

   However, in normal cases, this should work fine, in particular when there
   is only one instance of relayer performing the client update.
*/
pub struct QueryLatestConsensusStateHeightAsHeights;

#[cgp_provider(ConsensusStateHeightsQuerierComponent)]
impl<Chain, Counterparty> ConsensusStateHeightsQuerier<Chain, Counterparty>
    for QueryLatestConsensusStateHeightAsHeights
where
    Chain: CanQueryClientStateWithLatestHeight<Counterparty>,
    Counterparty: HasHeightType + HasClientStateFields<Chain>,
{
    async fn query_consensus_state_heights(
        chain: &Chain,
        client_id: &Chain::ClientId,
    ) -> Result<Vec<Counterparty::Height>, Chain::Error> {
        let client_state = chain
            .query_client_state_with_latest_height(PhantomData, client_id)
            .await?;

        let latest_height = Counterparty::client_state_latest_height(&client_state);

        Ok(vec![latest_height])
    }
}
