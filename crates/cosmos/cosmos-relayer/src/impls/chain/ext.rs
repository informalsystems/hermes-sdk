use async_trait::async_trait;

use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_client_components::traits::grpc_address::HasGrpcAddress;
use hermes_cosmos_client_components::traits::has_tx_context::HasTxContext;
use hermes_cosmos_client_components::traits::rpc_client::HasRpcClient;
use http::Uri;
use ibc_relayer::chain::handle::BaseChainHandle;
use tendermint_rpc::{HttpClient, Url};

use crate::contexts::chain::CosmosChain;
use crate::contexts::transaction::CosmosTxContext;
use crate::types::error::{BaseError, Error};

impl HasTxContext for CosmosChain {
    type TxContext = CosmosTxContext;

    fn tx_context(&self) -> &Self::TxContext {
        &self.tx_context
    }
}

impl HasGrpcAddress for CosmosChain {
    fn grpc_address(&self) -> &Uri {
        &self.tx_context.tx_config.grpc_address
    }
}

impl HasRpcClient for CosmosChain {
    fn rpc_client(&self) -> &HttpClient {
        &self.tx_context.rpc_client
    }

    fn rpc_address(&self) -> &Url {
        &self.tx_context.tx_config.rpc_address
    }
}

#[async_trait]
impl HasBlockingChainHandle for CosmosChain {
    type ChainHandle = BaseChainHandle;

    async fn with_blocking_chain_handle<R>(
        &self,
        cont: impl FnOnce(BaseChainHandle) -> Result<R, Error> + Send + 'static,
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
