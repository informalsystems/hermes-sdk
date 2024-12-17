use cgp::core::error::CanRaiseError;
use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerier;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use tendermint::Hash as TxHash;
use tendermint_rpc::endpoint::tx::Response as TxResponse;
use tendermint_rpc::query::Query;
use tendermint_rpc::{Client, Error as TendermintRpcError, Order};

use crate::traits::rpc_client::HasRpcClient;

pub struct QueryCosmosTxResponse;

impl<Chain> TxResponseQuerier<Chain> for QueryCosmosTxResponse
where
    Chain: HasTransactionHashType<TxHash = TxHash>
        + HasTxResponseType<TxResponse = TxResponse>
        + HasRpcClient
        + CanRaiseError<TendermintRpcError>,
{
    async fn query_tx_response(
        chain: &Chain,
        tx_hash: &TxHash,
    ) -> Result<Option<TxResponse>, Chain::Error> {
        let query = Query::eq("tx.hash", tx_hash.to_string());

        let response = chain
            .rpc_client()
            .tx_search(
                query,
                false,
                1,
                1, // get only the first Tx matching the query
                Order::Ascending,
            )
            .await
            .map_err(Chain::raise_error)?;

        Ok(response.txs.into_iter().next())
    }
}
