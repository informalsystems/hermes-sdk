use cgp::core::error::CanRaiseError;
use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerier;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use ibc_relayer::chain::cosmos::query::tx::query_tx_response;
use ibc_relayer::error::Error as RelayerError;
use tendermint::Hash as TxHash;
use tendermint_rpc::endpoint::tx::Response as TxResponse;

use crate::traits::rpc_client::HasRpcClient;

pub struct QueryCosmosTxResponse;

impl<Chain> TxResponseQuerier<Chain> for QueryCosmosTxResponse
where
    Chain: HasTransactionHashType<TxHash = TxHash>
        + HasTxResponseType<TxResponse = TxResponse>
        + HasRpcClient
        + CanRaiseError<RelayerError>,
{
    async fn query_tx_response(
        chain: &Chain,
        tx_hash: &TxHash,
    ) -> Result<Option<TxResponse>, Chain::Error> {
        let response = query_tx_response(chain.rpc_client(), chain.rpc_address(), tx_hash)
            .await
            .map_err(Chain::raise_error)?;

        Ok(response)
    }
}
