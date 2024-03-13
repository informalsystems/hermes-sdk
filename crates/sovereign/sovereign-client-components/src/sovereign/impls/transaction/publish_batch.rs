use cgp_core::CanRaiseError;
use hermes_relayer_components::transaction::traits::types::HasTransactionType;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;

use crate::sovereign::traits::rollup::json_rpc_client::HasJsonRpcClient;
use crate::sovereign::traits::rollup::publish_batch::TransactionBatchPublisher;

pub struct PublishSovereignTransactionBatch;

impl<Chain> TransactionBatchPublisher<Chain> for PublishSovereignTransactionBatch
where
    Chain: HasJsonRpcClient
        + HasTransactionType<Transaction = Vec<u8>>
        + CanRaiseError<ClientError>
        + CanRaiseError<serde_json::Error>,
    Chain::JsonRpcClient: ClientT,
{
    async fn publish_transaction_batch(
        chain: &Chain,
        transactions: &[Vec<u8>],
    ) -> Result<(), Chain::Error> {
        let rpc_client = chain.json_rpc_client();

        let response: serde_json::Value = rpc_client
            .request("sequencer_publishBatch", transactions)
            .await
            .map_err(Chain::raise_error)?;

        println!("sequencer_publishBatch response: {}", response);

        Ok(())
    }
}
