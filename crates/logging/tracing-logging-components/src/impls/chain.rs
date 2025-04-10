use core::fmt::{Debug, Display};

use cgp::prelude::*;
use hermes_logging_components::traits::logger::{Logger, LoggerComponent};
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use hermes_relayer_components::transaction::impls::poll_tx_response::{
    LogRetryQueryTxResponse, TxNoResponseError,
};
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTxHashType;
use tracing::{debug, error, trace};

use crate::contexts::logger::TracingLogger;

#[cgp_provider(LoggerComponent)]
impl<'a, Chain> Logger<Chain, LogSendMessagesWithSignerAndNonce<'a, Chain>> for TracingLogger
where
    Chain: HasSignerType + HasNonceType + HasMessageType + HasChainId,
    Chain::Signer: Debug,
    Chain::Nonce: Debug,
{
    async fn log(
        chain: &Chain,
        message: &str,
        details: &LogSendMessagesWithSignerAndNonce<'a, Chain>,
    ) {
        trace!(
            target: "hermes::tx",
            chain_id = %chain.chain_id(),
            nonce = ?details.nonce,
            signer = ?details.signer,
            "{message}",
        );
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Chain> Logger<Chain, TxNoResponseError<'a, Chain>> for TracingLogger
where
    Chain: HasTxHashType + HasChainId,
    Chain::TxHash: Display,
{
    async fn log(chain: &Chain, message: &str, details: &TxNoResponseError<'a, Chain>) {
        error!(
            target: "hermes::tx",
            chain_id = %chain.chain_id(),
            tx_hash = %details.tx_hash,
            wait_timeout = ?details.wait_timeout,
            elapsed = ?details.elapsed,
            "{message}",
        );
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Chain> Logger<Chain, LogRetryQueryTxResponse<'a, Chain>> for TracingLogger
where
    Chain: HasTxHashType + HasChainId + HasAsyncErrorType,
    Chain::TxHash: Display,
    Chain::Error: Debug,
{
    async fn log(chain: &Chain, message: &str, details: &LogRetryQueryTxResponse<'a, Chain>) {
        debug!(
            target: "hermes::tx",
            chain_id = %chain.chain_id(),
            tx_hash = %details.tx_hash,
            elapsed = ?details.elapsed,
            error = ?details.error,
            "{message}",
        );
    }
}
