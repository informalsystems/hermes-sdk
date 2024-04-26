use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightsQuerier;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{PageRequest, QueryConsensusStateHeightsRequest};
use ibc_relayer::error::Error as RelayerError;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;

use crate::traits::chain_handle::HasBlockingChainHandle;

pub struct QueryConsensusStateHeightsFromChainHandle;

impl<Chain, Counterparty> ConsensusStateHeightsQuerier<Chain, Counterparty>
    for QueryConsensusStateHeightsFromChainHandle
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + HasBlockingChainHandle
        + CanRaiseError<RelayerError>,
    Counterparty: HasHeightType<Height = Height>,
{
    async fn query_consensus_state_heights(
        chain: &Chain,
        client_id: &ClientId,
    ) -> Result<Vec<Height>, Chain::Error> {
        let client_id = client_id.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                chain_handle
                    .query_consensus_state_heights(QueryConsensusStateHeightsRequest {
                        client_id: client_id.clone(),
                        pagination: Some(PageRequest::all()),
                    })
                    .map_err(Chain::raise_error)
            })
            .await
    }
}
