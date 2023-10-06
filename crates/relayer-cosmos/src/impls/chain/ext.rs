use cgp_core::Async;
use http::Uri;

use crate::contexts::chain::CosmosChain;
use crate::contexts::transaction::CosmosTxContext;
use crate::traits::grpc_address::HasGrpcAddress;
use crate::traits::has_tx_context::HasTxContext;

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
