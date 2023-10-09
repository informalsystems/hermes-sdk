use cgp_core::Async;
use http::Uri;
use tendermint_rpc::HttpClient;

use crate::contexts::chain::CosmosChain;
use crate::contexts::transaction::CosmosTxContext;
use crate::traits::grpc_address::HasGrpcAddress;
use crate::traits::has_tx_context::HasTxContext;
use crate::traits::rpc_client::HasRpcClient;

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
}
