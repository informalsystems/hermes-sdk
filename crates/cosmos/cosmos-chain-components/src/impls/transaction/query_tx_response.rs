use cgp::extra::runtime::HasRuntime;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::{
    CanQueryChainHeight, HasHeightFields, HasPollInterval,
};
use hermes_relayer_components::transaction::traits::{
    HasTxHashType, HasTxResponseType, TxResponseQuerier, TxResponseQuerierComponent,
};
use hermes_runtime_components::traits::sleep::CanSleep;
use tendermint::Hash as TxHash;
use tendermint_rpc::endpoint::tx::Response as TxResponse;
use tendermint_rpc::query::Query;
use tendermint_rpc::{Client, Error as TendermintRpcError, Order};

use crate::traits::rpc_client::HasRpcClient;

#[cgp_new_provider(TxResponseQuerierComponent)]
impl<Chain> TxResponseQuerier<Chain> for QueryCosmosTxResponse
where
    Chain: HasRuntime
        + HasTxHashType<TxHash = TxHash>
        + HasTxResponseType<TxResponse = TxResponse>
        + HasRpcClient
        + CanQueryChainHeight
        + HasHeightFields
        + HasPollInterval
        + CanRaiseAsyncError<TendermintRpcError>,
    Chain::Runtime: CanSleep,
{
    async fn query_tx_response(
        chain: &Chain,
        tx_hash: &TxHash,
    ) -> Result<Option<TxResponse>, Chain::Error> {
        let query = Query::eq("tx.hash", tx_hash.to_string());

        let responses = chain
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

        let m_response = responses.txs.into_iter().next();

        match m_response {
            Some(response) => {
                // Try to ensure that the chain progress to a later height than the height that
                // returns the response, so that side effects such as merkle proofs can always
                // be queried from the chain thereafter.
                //
                // This is usually not an issue in local tests. But when interacting with public
                // RPC servers, at the first moment the transaction becomes available, it is not
                // always propogated to all full nodes yet. This would result in the relayer
                // interacting with a past chain state that did not have the transaction available
                // yet, resulting in errors such as ack not found.
                //
                // The best way to work around this is to have the relayer always interact with only
                // one dedicated full node. That way, when the full node says that the transaction
                // is already there, then it is guaranteed to always available there onward.
                // But we need this workaround so that the relayer still works with multiple full
                // nodes and with public RPC endpoints.
                for _ in 0..20 {
                    let response_height: u64 = response.height.into();
                    let chain_height = chain.query_chain_height().await?;

                    // We can technically wait for the same height with >=, but we use > to be
                    // a little safer to ensure that all full nodes have processed the transaction.
                    if Chain::revision_height(&chain_height) > response_height {
                        break;
                    } else {
                        chain.runtime().sleep(chain.poll_interval()).await;
                    }
                }

                Ok(Some(response))
            }
            None => Ok(None),
        }
    }
}
