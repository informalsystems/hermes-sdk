use core::fmt::Debug;
use core::time::Duration;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;

use crate::logger::traits::level::HasBaseLogLevels;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::sleep::CanSleep;
use crate::runtime::traits::time::HasTime;
use crate::transaction::traits::logs::logger::CanLogTx;
use crate::transaction::traits::poll_tx_response::TxResponsePoller;
use crate::transaction::traits::query_tx_response::CanQueryTxResponse;
use crate::transaction::traits::types::tx_hash::HasTransactionHashType;

pub struct PollTxResponse;

pub struct TxNoResponseError<'a, Chain>
where
    Chain: HasTransactionHashType,
{
    pub chain: &'a Chain,
    pub tx_hash: &'a Chain::TxHash,
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

#[derive_component(PollTimeoutGetterComponent, PollTimeoutGetter<Chain>)]
pub trait HasPollTimeout {
    fn poll_timeout(&self) -> Duration;

    fn poll_backoff(&self) -> Duration;
}

impl<Chain> TxResponsePoller<Chain> for PollTxResponse
where
    Chain: CanLogTx
        + CanQueryTxResponse
        + HasPollTimeout
        + HasRuntime
        + for<'a> CanRaiseError<TxNoResponseError<'a, Chain>>,
    Chain::Runtime: HasTime + CanSleep,
{
    async fn poll_tx_response(
        chain: &Chain,
        tx_hash: &Chain::TxHash,
    ) -> Result<Chain::TxResponse, Chain::Error> {
        let runtime = chain.runtime();
        let wait_timeout = chain.poll_timeout();
        let wait_backoff = chain.poll_backoff();

        let start_time = runtime.now();

        loop {
            let response = chain.query_tx_response(tx_hash).await;

            match response {
                Ok(None) => {
                    let elapsed = Chain::Runtime::duration_since(&start_time, &runtime.now());
                    if elapsed > wait_timeout {
                        chain.log_tx(
                            Chain::Logger::LEVEL_ERROR,
                            "no tx response received, and poll timeout has recached. returning error",
                            |log| {
                                log.debug("elapsed", &elapsed).debug("wait_timeout", &wait_timeout);
                            }
                        );

                        return Err(Chain::raise_error(TxNoResponseError { chain, tx_hash }));
                    } else {
                        runtime.sleep(wait_backoff).await;
                    }
                }
                Ok(Some(response)) => {
                    chain.log_tx(
                        Chain::Logger::LEVEL_TRACE,
                        "received tx response, finish polling",
                        |_| {},
                    );

                    return Ok(response);
                }
                Err(e) => {
                    chain.log_tx(
                        Chain::Logger::LEVEL_ERROR,
                        "query_tx_response returned error",
                        |log| {
                            log.debug("error", &e);
                        },
                    );

                    /*
                        If querying the TX response returns failure, it might be a temporary network
                        failure that can be recovered later on. Hence it would not be good if
                        we return error immediately, as we may still have the chance to get a
                        proper transaction response later on.

                        However, if the query still returns error after the wait timeout exceeded,
                        we return the error we get from the query.

                        TODO: check whether the error is retryable before re-polling.
                    */

                    let elapsed = Chain::Runtime::duration_since(&start_time, &runtime.now());
                    if elapsed > wait_timeout {
                        return Err(e);
                    } else {
                        runtime.sleep(wait_backoff).await;
                    }
                }
            }
        }
    }
}
