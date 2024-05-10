use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::queries::connection_end::ConnectionEndQuerier;
use hermes_relayer_components::chain::traits::types::connection::HasConnectionEndType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{IncludeProof, QueryConnectionRequest, QueryHeight};
use ibc_relayer_types::core::ics03_connection::connection::ConnectionEnd;
use ibc_relayer_types::core::ics24_host::identifier::ConnectionId;
use ibc_relayer_types::Height;

use crate::traits::chain_handle::HasBlockingChainHandle;

pub struct QueryCosmosConnectionEndFromChainHandle;

impl<Chain, Counterparty> ConnectionEndQuerier<Chain, Counterparty>
    for QueryCosmosConnectionEndFromChainHandle
where
    Chain: HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasIbcChainTypes<Counterparty, Height = Height, ConnectionId = ConnectionId>
        + HasBlockingChainHandle
        + HasErrorType,
{
    async fn query_connection_end(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        height: &Height,
    ) -> Result<Chain::ConnectionEnd, Chain::Error> {
        let connection_id = connection_id.clone();
        let height = height.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let (connection_end, _) = chain_handle
                    .query_connection(
                        QueryConnectionRequest {
                            connection_id: connection_id.clone(),
                            height: QueryHeight::Specific(height),
                        },
                        IncludeProof::No,
                    )
                    .map_err(Chain::raise_error)?;

                Ok(connection_end)
            })
            .await
    }
}
