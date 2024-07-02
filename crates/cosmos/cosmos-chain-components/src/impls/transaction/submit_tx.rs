use core::fmt::Debug;

use cgp_core::error::CanRaiseError;
use hermes_relayer_components::transaction::traits::submit_tx::TxSubmitter;
use hermes_relayer_components::transaction::traits::types::transaction::HasTransactionType;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use ibc_proto::cosmos::tx::v1beta1::TxRaw;
use ibc_relayer::chain::cosmos::types::tx::SignedTx;
use prost::Message;
use tendermint::Hash as TxHash;
use tendermint_rpc::endpoint::broadcast::tx_sync::Response;
use tendermint_rpc::{Client, Error as RpcError};

use crate::traits::rpc_client::HasRpcClient;

pub struct BroadcastCosmosTx;

pub struct BroadcastTxError<'a, Chain> {
    pub chain: &'a Chain,
    pub tx: &'a SignedTx,
    pub response: Response,
}

impl<'a, Chain> Debug for BroadcastTxError<'a, Chain> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BroadcastTxError")
            .field("response", &self.response)
            .finish()
    }
}

impl<Chain> TxSubmitter<Chain> for BroadcastCosmosTx
where
    Chain: HasTransactionType<Transaction = SignedTx>
        + HasTransactionHashType<TxHash = TxHash>
        + HasRpcClient
        + CanRaiseError<RpcError>
        + for<'a> CanRaiseError<BroadcastTxError<'a, Chain>>,
{
    async fn submit_tx(chain: &Chain, tx: &SignedTx) -> Result<TxHash, Chain::Error> {
        let tx_raw = TxRaw {
            body_bytes: tx.body_bytes.clone(),
            auth_info_bytes: tx.auth_info_bytes.clone(),
            signatures: tx.signatures.clone(),
        };

        let tx_bytes = Message::encode_to_vec(&tx_raw);

        let response = chain
            .rpc_client()
            .broadcast_tx_sync(tx_bytes)
            .await
            .map_err(Chain::raise_error)?;

        if response.code.is_err() {
            Err(Chain::raise_error(BroadcastTxError {
                chain,
                tx,
                response,
            }))
        } else {
            Ok(response.hash)
        }
    }
}
