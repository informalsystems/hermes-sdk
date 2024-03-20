use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_client_components::traits::grpc_address::GrpcAddressGetter;
use hermes_cosmos_client_components::traits::rpc_client::RpcClientGetter;
use http::Uri;
use ibc_relayer::chain::handle::BaseChainHandle;
use tendermint_rpc::{HttpClient, Url};

use crate::chain::components::CosmosChainComponents;
use crate::contexts::chain::CosmosChain;
use crate::types::error::Error;

impl GrpcAddressGetter<CosmosChain> for CosmosChainComponents {
    fn grpc_address(chain: &CosmosChain) -> &Uri {
        &chain.tx_config.grpc_address
    }
}

impl RpcClientGetter<CosmosChain> for CosmosChainComponents {
    fn rpc_client(chain: &CosmosChain) -> &HttpClient {
        &chain.rpc_client
    }

    fn rpc_address(chain: &CosmosChain) -> &Url {
        &chain.tx_config.rpc_address
    }
}

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
            .await?
    }
}
