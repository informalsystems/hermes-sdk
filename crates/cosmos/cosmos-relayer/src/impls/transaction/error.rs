use eyre::eyre;
use hermes_relayer_components::transaction::components::poll_tx_response::CanRaiseNoTxResponseError;
use tendermint::Hash as TxHash;

use crate::contexts::transaction::CosmosTxContext;
use crate::types::error::Error;

impl CanRaiseNoTxResponseError for CosmosTxContext {
    fn tx_no_response_error(tx_hash: &TxHash) -> Error {
        eyre!("failed to receive tx response for tx hash: {}", tx_hash).into()
    }
}
