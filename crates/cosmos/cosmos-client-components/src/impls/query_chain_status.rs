use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::components::chain_status_querier::ChainStatusQuerier;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use ibc_relayer::chain::endpoint::ChainStatus;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::Height;
use tendermint_rpc::Client;
use tendermint_rpc::Error as TendermintRpcError;

use crate::traits::rpc_client::HasRpcClient;

pub struct QueryChainStatusWithChainHandle;

#[async_trait]
impl<Chain> ChainStatusQuerier<Chain> for QueryChainStatusWithChainHandle
where
    Chain: HasErrorType
        + HasChainStatusType<ChainStatus = ChainStatus>
        + HasRpcClient
        + CanRaiseError<TendermintRpcError>
        + CanRaiseError<Ics02Error>,
{
    async fn query_chain_status(chain: &Chain) -> Result<Chain::ChainStatus, Chain::Error> {
        let rpc_client = chain.rpc_client();

        let abci_info = rpc_client.abci_info().await.map_err(Chain::raise_error)?;

        let response = rpc_client
            .header(abci_info.last_block_height)
            .await
            .map_err(Chain::raise_error)?;

        let height = Height::new(
            ChainId::chain_version(response.header.chain_id.as_str()),
            u64::from(abci_info.last_block_height),
        )
        .map_err(Chain::raise_error)?;

        let timestamp = response.header.time.into();

        Ok(ChainStatus { height, timestamp })
    }
}
