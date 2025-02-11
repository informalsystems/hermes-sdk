use core::fmt::Debug;
use core::time::Duration;

use cgp::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_runtime_components::traits::time::HasTime;

use crate::error::traits::retry::HasRetryableError;
use crate::transaction::traits::poll_tx_response::{TxResponsePoller, TxResponsePollerComponent};
use crate::transaction::traits::query_tx_response::CanQueryTxResponse;
use crate::transaction::traits::types::tx_hash::HasTransactionHashType;

pub struct PollTxResponse;

pub struct TxNoResponseError<'a, Chain>
where
    Chain: HasTransactionHashType,
{
    pub chain: &'a Chain,
    pub tx_hash: &'a Chain::TxHash,
    pub wait_timeout: &'a Duration,
    pub elapsed: &'a Duration,
}

pub struct LogRetryQueryTxResponse<'a, Chain>
where
    Chain: HasTransactionHashType + HasAsyncErrorType,
{
    pub chain: &'a Chain,
    pub tx_hash: &'a Chain::TxHash,
    pub elapsed: &'a Duration,
    pub error: &'a Chain::Error,
}

impl<'a, Chain> Debug for TxNoResponseError<'a, Chain>
where
    Chain: HasTransactionHashType,
    Chain::TxHash: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TxNoResponseError")
            .field("tx_hash", &self.tx_hash)
            .finish()
    }
}

#[cgp_component {
  provider: PollTimeoutGetter,
  context: Chain,
}]
pub trait HasPollTimeout {
    fn poll_timeout(&self) -> Duration;

    fn poll_backoff(&self) -> Duration;
}

#[cgp_provider(TxResponsePollerComponent)]
impl<Chain> TxResponsePoller<Chain> for PollTxResponse
where
    Chain: CanQueryTxResponse
        + HasPollTimeout
        + HasRuntime
        + HasLogger
        + HasRetryableError
        + for<'a> CanRaiseAsyncError<TxNoResponseError<'a, Chain>>,
    Chain::Runtime: HasTime + CanSleep,
    Chain::Logger: for<'a> CanLog<TxNoResponseError<'a, Chain>>
        + for<'a> CanLog<LogRetryQueryTxResponse<'a, Chain>>,
{
    async fn poll_tx_response(
        chain: &Chain,
        tx_hash: &Chain::TxHash,
    ) -> Result<Chain::TxResponse, Chain::Error> {
        let runtime = chain.runtime();
        let wait_timeout = chain.poll_timeout();
        let wait_backoff = chain.poll_backoff();

        let start_time = runtime.now();

        let logger = chain.logger();

        loop {
            let response = chain.query_tx_response(tx_hash).await;

            match response {
                Ok(Some(response)) => {
                    return Ok(response);
                }
                Ok(None) => {
                    let elapsed = Chain::Runtime::duration_since(&start_time, &runtime.now());
                    if elapsed > wait_timeout {
                        let e = TxNoResponseError {
                            chain,
                            tx_hash,
                            elapsed: &elapsed,
                            wait_timeout: &wait_timeout,
                        };

                        logger.log("no tx response received, and poll timeout has reached. returning error", &e).await;

                        return Err(Chain::raise_error(e));
                    } else {
                        runtime.sleep(wait_backoff).await;
                    }
                }
                Err(e) => {
                    if !Chain::is_retryable_error(&e) {
                        return Err(e);
                    }

                    /*
                        If querying the TX response returns failure, it might be a temporary network
                        failure that can be recovered later on. Hence it would not be good if
                        we return error immediately, as we may still have the chance to get a
                        proper transaction response later on.

                        However, if the query still returns error after the wait timeout exceeded,
                        we return the error we get from the query.
                    */

                    let elapsed = Chain::Runtime::duration_since(&start_time, &runtime.now());
                    if elapsed > wait_timeout {
                        return Err(e);
                    } else {
                        logger
                            .log(
                                "retry polling with query_tx_response returning retryable error",
                                &LogRetryQueryTxResponse {
                                    chain,
                                    tx_hash,
                                    elapsed: &elapsed,
                                    error: &e,
                                },
                            )
                            .await;

                        runtime.sleep(wait_backoff).await;
                    }
                }
            }
        }
    }
}
