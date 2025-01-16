use core::fmt::{Debug, Display};

use cgp::prelude::{Async, HasAsyncErrorType};
use hermes_logging_components::traits::logger::Logger;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use hermes_relayer_components::transaction::impls::poll_tx_response::{
    LogRetryQueryTxResponse, TxNoResponseError,
};
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use tracing::{debug, error, trace};

use crate::contexts::logger::TracingLogger;

impl<'a, Logging, Chain> Logger<Logging, LogSendMessagesWithSignerAndNonce<'a, Chain>>
    for TracingLogger
where
    Logging: Async,
    Chain: HasSignerType + HasNonceType + HasMessageType + HasChainId,
    Chain::Signer: Debug,
    Chain::Nonce: Debug,
{
    async fn log(
        _logging: &Logging,
        message: &str,
        details: &LogSendMessagesWithSignerAndNonce<'a, Chain>,
    ) {
        trace!(
            chain_id = %details.chain.chain_id(),
            nonce = ?details.nonce,
            signer = ?details.signer,
            "{message}",
        );
    }
}

impl<'a, Logging, Chain> Logger<Logging, TxNoResponseError<'a, Chain>> for TracingLogger
where
    Logging: Async,
    Chain: HasTransactionHashType + HasChainId,
    Chain::TxHash: Display,
{
    async fn log(_logging: &Logging, message: &str, details: &TxNoResponseError<'a, Chain>) {
        error!(
            chain_id = %details.chain.chain_id(),
            tx_hash = %details.tx_hash,
            wait_timeout = ?details.wait_timeout,
            elapsed = ?details.elapsed,
            "{message}",
        );
    }
}

impl<'a, Logging, Chain> Logger<Logging, LogRetryQueryTxResponse<'a, Chain>> for TracingLogger
where
    Logging: Async,
    Chain: HasTransactionHashType + HasChainId + HasAsyncErrorType,
    Chain::TxHash: Display,
    Chain::Error: Debug,
{
    async fn log(_logging: &Logging, message: &str, details: &LogRetryQueryTxResponse<'a, Chain>) {
        debug!(
            chain_id = %details.chain.chain_id(),
            tx_hash = %details.tx_hash,
            elapsed = ?details.elapsed,
            error = ?details.error,
            "{message}",
        );
    }
}
