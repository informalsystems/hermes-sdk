use eyre::eyre;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{PageRequest, QueryConsensusStateHeightsRequest};
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;

use crate::contexts::chain::CosmosChain;
use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::types::error::{BaseError, Error};

pub async fn find_consensus_state_height_before<Chain: ChainHandle>(
    chain: &CosmosChain<Chain>,
    client_id: &ClientId,
    target_height: &Height,
) -> Result<Height, Error> {
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
                    .map_err(BaseError::relayer)?;

                heights.sort_by_key(|&h| core::cmp::Reverse(h));

                heights
            };

            let height = heights
                .into_iter()
                .find(|height| height < &target_height)
                .ok_or_else(|| {
                    BaseError::generic(eyre!(
                        "no consensus state found that is smaller than target height {}",
                        target_height
                    ))
                })?;

            Ok(height)
        })
        .await
}
