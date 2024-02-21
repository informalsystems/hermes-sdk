use cgp_core::prelude::*;
use hermes_relayer_components::transaction::traits::types::HasTransactionType;

#[derive_component(TransactionBatchPublisherComponent, TransactionBatchPublisher<Chain>)]
#[async_trait]
pub trait CanPublishTransactionBatch: HasTransactionType + HasErrorType {
    async fn publish_transaction_batch(
        &self,
        transactions: &[Self::Transaction],
    ) -> Result<(), Self::Error>;
}
