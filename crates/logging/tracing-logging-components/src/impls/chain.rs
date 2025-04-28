use core::fmt::{Debug, Display};

use hermes_logging_components::traits::{Logger, LoggerComponent};
use hermes_prelude::*;
use hermes_relayer_components::chain::traits::{HasChainId, HasMessageType};
use hermes_relayer_components::transaction::impls::{
    LogRetryQueryTxResponse, LogSendMessagesWithSignerAndNonce, TxNoResponseError,
};
use hermes_relayer_components::transaction::traits::{HasNonceType, HasSignerType, HasTxHashType};
use tracing::{debug, error, trace};

use crate::contexts::TracingLogger;

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
