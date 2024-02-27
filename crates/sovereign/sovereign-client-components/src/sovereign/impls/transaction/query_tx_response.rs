use cgp_core::CanRaiseError;
use hermes_relayer_components::transaction::traits::components::tx_response_querier::TxResponseQuerier;
use hermes_relayer_components::transaction::traits::types::{
    HasTransactionHashType, HasTxResponseType,
};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;

use crate::sovereign::traits::rollup::json_rpc_client::HasJsonRpcClient;
use crate::sovereign::types::rpc::tx_hash::TxHash;
use crate::sovereign::types::rpc::tx_response::TxResponse;

pub struct QuerySovereignTxResponse;

impl<Chain> TxResponseQuerier<Chain> for QuerySovereignTxResponse
where
    Chain: HasTxResponseType<TxResponse = TxResponse>
        + HasTransactionHashType<TxHash = TxHash>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>,
{
    async fn query_tx_response(
        chain: &Chain,
        tx_hash: &TxHash,
    ) -> Result<Option<TxResponse>, Chain::Error> {
        let response = chain
            .json_rpc_client()
            .request("ledger_getTransactionByHash", (&tx_hash,))
            .await
            .map_err(Chain::raise_error)?;

        Ok(response)
    }
}
