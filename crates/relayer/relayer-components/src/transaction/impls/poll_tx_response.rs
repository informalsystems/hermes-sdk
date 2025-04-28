use core::fmt::Debug;
use core::marker::PhantomData;
use core::time::Duration;

use hermes_chain_components::traits::HasPollInterval;
use hermes_logging_components::traits::CanLog;
use hermes_prelude::*;
use hermes_runtime_components::traits::{CanSleep, HasRuntime, HasTime};

use crate::error::traits::HasRetryableError;
use crate::transaction::traits::{
    CanQueryTxResponse, HasTxHashType, TxResponsePoller, TxResponsePollerComponent,
};

pub struct PollTxResponse;

pub struct TxNoResponseError<'a, Chain>
where
    Chain: HasTxHashType,
{
    pub tx_hash: &'a Chain::TxHash,
    pub wait_timeout: &'a Duration,
    pub elapsed: &'a Duration,
}

pub struct LogRetryQueryTxResponse<'a, Chain>
where
    Chain: HasTxHashType + HasAsyncErrorType,
{
    pub tx_hash: &'a Chain::TxHash,
    pub elapsed: &'a Duration,
    pub error: &'a Chain::Error,
}

impl<'a, Chain> Debug for TxNoResponseError<'a, Chain>
where
    Chain: HasTxHashType,
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
}

#[cgp_provider(PollTimeoutGetterComponent)]
impl<Context, Tag> PollTimeoutGetter<Context> for UseField<Tag>
where
    Context: HasField<Tag, Value = Duration>,
{
    fn poll_timeout(context: &Context) -> Duration {
        *context.get_field(PhantomData)
    }
}

#[cgp_provider(TxResponsePollerComponent)]
impl<Chain> TxResponsePoller<Chain> for PollTxResponse
where
    Chain: CanQueryTxResponse
        + HasPollTimeout
        + HasPollInterval
        + HasRuntime
        + HasRetryableError
        + for<'a> CanLog<TxNoResponseError<'a, Chain>>
        + for<'a> CanLog<LogRetryQueryTxResponse<'a, Chain>>
        + for<'a> CanRaiseAsyncError<TxNoResponseError<'a, Chain>>,
    Chain::Runtime: HasTime + CanSleep,
{
    async fn poll_tx_response(
        chain: &Chain,
        tx_hash: &Chain::TxHash,
    ) -> Result<Chain::TxResponse, Chain::Error> {
        let runtime = chain.runtime();
        let wait_timeout = chain.poll_timeout();
        let wait_backoff = chain.poll_interval();

        let start_time = runtime.now();

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
                            tx_hash,
                            elapsed: &elapsed,
                            wait_timeout: &wait_timeout,
                        };

                        chain.log("no tx response received, and poll timeout has reached. returning error", &e).await;

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
                        chain
                            .log(
                                "retry polling with query_tx_response returning retryable error",
                                &LogRetryQueryTxResponse {
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
