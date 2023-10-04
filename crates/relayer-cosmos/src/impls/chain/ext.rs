use cgp_core::Async;

use crate::contexts::chain::CosmosChain;
use crate::contexts::transaction::CosmosTxContext;
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
