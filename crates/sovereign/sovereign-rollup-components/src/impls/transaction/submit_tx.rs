use cgp_core::CanRaiseError;
use hermes_relayer_components::transaction::traits::submit_tx::TxSubmitter;
use hermes_relayer_components::transaction::traits::types::transaction::HasTransactionType;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;

use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::tx::tx_hash::TxHash;

pub struct SubmitSovereignTransaction;

impl<Chain> TxSubmitter<Chain> for SubmitSovereignTransaction
where
    Chain: HasJsonRpcClient
        + HasTransactionType<Transaction = Vec<u8>>
        + HasTransactionHashType<TxHash = TxHash>
        + CanRaiseError<ClientError>
        + CanRaiseError<serde_json::Error>,
    Chain::JsonRpcClient: ClientT,
{
    async fn submit_tx(chain: &Chain, transaction: &Vec<u8>) -> Result<TxHash, Chain::Error> {
        let rpc_client = chain.json_rpc_client();

        let _response: serde_json::Value = rpc_client
            .request("sequencer_publishBatch", [transaction])
            .await
            .map_err(Chain::raise_error)?;

        let tx_hash = TxHash::from_signed_tx_bytes(transaction);

        Ok(tx_hash)
    }
}
