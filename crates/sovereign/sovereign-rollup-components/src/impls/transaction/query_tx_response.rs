use core::ops::Range;

use cgp_core::CanRaiseError;
use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerier;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use hex::ToHex;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Deserialize;

use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::tx::tx_hash::TxHash;
use crate::types::tx::tx_response::{TxEffect, TxResponse};

pub struct QuerySovereignTxResponse;

impl<Chain> TxResponseQuerier<Chain> for QuerySovereignTxResponse
where
    Chain: HasTxResponseType<TxResponse = TxResponse>
        + HasTransactionHashType<TxHash = TxHash>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>,
    Chain::JsonRpcClient: ClientT,
{
    async fn query_tx_response(
        chain: &Chain,
        tx_hash: &TxHash,
    ) -> Result<Option<TxResponse>, Chain::Error> {
        let response: Option<QueryTxResponse> = chain
            .json_rpc_client()
            .request("ledger_getTransactionByHash", (&tx_hash,))
            .await
            .map_err(Chain::raise_error)?;

        if let Some(response) = response {
            let tx_hash_str = tx_hash.0.encode_hex::<String>();

            let events = chain
                .json_rpc_client()
                .request("ledger_getEventsByTxnHash", (tx_hash_str,))
                .await
                .map_err(Chain::raise_error)?;

            let response = TxResponse {
                hash: response.hash,
                events,
                custom_receipt: response.custom_receipt,
            };

            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryTxResponse {
    pub hash: TxHash,
    pub event_range: Range<u64>,
    pub custom_receipt: TxEffect,
}
