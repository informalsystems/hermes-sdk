use async_trait::async_trait;
use cgp_core::Async;
use http::Uri;
use ibc_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use ibc_relayer::chain::handle::ChainHandle;
use tendermint_rpc::{HttpClient, Url};

use crate::contexts::chain::CosmosChain;
use crate::contexts::transaction::CosmosTxContext;
use crate::types::error::{BaseError, Error};
use ibc_cosmos_client_components::traits::grpc_address::HasGrpcAddress;
use ibc_cosmos_client_components::traits::has_tx_context::HasTxContext;
use ibc_cosmos_client_components::traits::rpc_client::HasRpcClient;

impl<Chain> HasTxContext for CosmosChain<Chain>
where
    Chain: Async,
{
    type TxContext = CosmosTxContext;

    fn tx_context(&self) -> &Self::TxContext {
        &self.tx_context
    }
}

impl<Chain> HasGrpcAddress for CosmosChain<Chain>
where
    Chain: Async,
{
    fn grpc_address(&self) -> &Uri {
        &self.tx_context.tx_config.grpc_address
    }
}

impl<Chain> HasRpcClient for CosmosChain<Chain>
where
    Chain: Async,
{
    fn rpc_client(&self) -> &HttpClient {
        &self.tx_context.rpc_client
    }

    fn rpc_address(&self) -> &Url {
        &self.tx_context.tx_config.rpc_address
    }
}

#[async_trait]
impl<Chain> HasBlockingChainHandle for CosmosChain<Chain>
where
    Chain: ChainHandle,
{
    type ChainHandle = Chain;

    async fn with_blocking_chain_handle<R>(
        &self,
        cont: impl FnOnce(Chain) -> Result<R, Error> + Send + 'static,
    ) -> Result<R, Error>
    where
        R: Send + 'static,
    {
        let chain_handle = self.handle.clone();

        self.runtime
            .runtime
            .spawn_blocking(move || cont(chain_handle))
            .await
            .map_err(BaseError::join)?
    }
}
