use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use eyre::eyre;
use hermes_relayer_components::chain::traits::components::consensus_state_height_querier::ConsensusStateHeightQuerier;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{PageRequest, QueryConsensusStateHeightsRequest};
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;

use crate::traits::chain_handle::HasBlockingChainHandle;

pub struct QueryConsensusStateHeightFromChainHandle;

#[async_trait]
impl<Chain, Counterparty> ConsensusStateHeightQuerier<Chain, Counterparty>
    for QueryConsensusStateHeightFromChainHandle
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + HasBlockingChainHandle
        + CanRaiseError<eyre::Report>,
    Counterparty: HasHeightType<Height = Height>,
{
    async fn find_consensus_state_height_before(
        chain: &Chain,
        client_id: &Chain::ClientId,
        target_height: &Counterparty::Height,
    ) -> Result<Counterparty::Height, Chain::Error> {
        let client_id = client_id.clone();
        let target_height = *target_height;

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let heights = {
                    let mut heights = chain_handle
                        .query_consensus_state_heights(QueryConsensusStateHeightsRequest {
                            client_id,
                            pagination: Some(PageRequest::all()),
                        })
                        .map_err(Chain::raise_error)?;

                    heights.sort_by_key(|&h| core::cmp::Reverse(h));

                    heights
                };

                let height = heights
                    .into_iter()
                    .find(|height| height < &target_height)
                    .ok_or_else(|| {
                        Chain::raise_error(eyre!(
                            "no consensus state found that is smaller than target height {}",
                            target_height
                        ))
                    })?;

                Ok(height)
            })
            .await
    }
}
