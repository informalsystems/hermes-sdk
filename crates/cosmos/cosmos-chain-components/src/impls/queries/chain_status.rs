use hermes_core::relayer_components::chain::traits::{
    ChainStatusQuerier, ChainStatusQuerierComponent, HasChainStatusType,
};
use hermes_prelude::*;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::ChainId;
use tendermint_rpc::{Client, Error as TendermintRpcError};

use crate::traits::HasRpcClient;
use crate::types::ChainStatus;

pub struct QueryCosmosChainStatus;

#[cgp_provider(ChainStatusQuerierComponent)]
impl<Chain> ChainStatusQuerier<Chain> for QueryCosmosChainStatus
where
    Chain: HasAsyncErrorType
        + HasChainStatusType<ChainStatus = ChainStatus>
        + HasRpcClient
        + CanRaiseAsyncError<TendermintRpcError>
        + CanRaiseAsyncError<ClientError>
        + CanRaiseAsyncError<IdentifierError>,
{
    async fn query_chain_status(chain: &Chain) -> Result<ChainStatus, Chain::Error> {
        let rpc_client = chain.rpc_client();

        let abci_info = rpc_client.abci_info().await.map_err(Chain::raise_error)?;

        let response = rpc_client
            .header(abci_info.last_block_height)
            .await
            .map_err(Chain::raise_error)?;

        let chain_id =
            ChainId::new(response.header.chain_id.as_str()).map_err(Chain::raise_error)?;

        let height = Height::new(
            chain_id.revision_number(),
            u64::from(abci_info.last_block_height),
        )
        .map_err(Chain::raise_error)?;

        let time = response.header.time;

        Ok(ChainStatus { height, time })
    }
}
