use core::fmt::Debug;
use core::time::Duration;

use cgp::extra::runtime::HasRuntime;
use hermes_logging_components::traits::CanLog;
use hermes_prelude::*;
use hermes_runtime_components::traits::CanSleep;

use crate::error::traits::{AsyncCont, HasRetryableError, RetryPerformer, RetryPerformerComponent};

#[cgp_new_provider(RetryPerformerComponent)]
impl<Context> RetryPerformer<Context> for PerformRetryWithRetryableError
where
    Context: HasRuntime
        + HasRetryableError
        + for<'a> CanLog<LogPerformRetry<'a, Context>>
        + for<'a> CanWrapAsyncError<ErrMaxRetryExceeded<'a, Context>>,
    Context::Runtime: CanSleep,
{
    async fn perform_with_retry<T: Send + Sync>(
        context: &Context,
        task_name: &str,
        max_retries: usize,
        cont: impl AsyncCont<Result<T, Context::Error>>,
    ) -> Result<T, Context::Error> {
        let runtime = context.runtime();

        let mut attempts: usize = 0;
        let mut retry_interval = Duration::from_millis(500);

        loop {
            let res = cont.run().await;

            match res {
                Ok(res) => {
                    return Ok(res);
                }
                Err(e) => {
                    if !Context::is_retryable_error(&e) {
                        return Err(e);
                    } else if attempts >= max_retries {
                        return Err(Context::wrap_error(
                            e,
                            ErrMaxRetryExceeded {
                                context,
                                task_name,
                                max_retries,
                            },
                        ));
                    } else {
                        context
                            .log(
                                "sleeping and retrying operation after encountering error",
                                &LogPerformRetry {
                                    context,
                                    error: &e,
                                    task_name,
                                    attempts,
                                    max_retries,
                                    retry_interval,
                                },
                            )
                            .await;

                        runtime.sleep(retry_interval).await;
                        attempts += 1;
                        retry_interval *= 2;
                    }
                }
            }
        }
    }
}

pub struct LogPerformRetry<'a, Context>
where
    Context: HasErrorType,
{
    pub context: &'a Context,
    pub error: &'a Context::Error,
    pub task_name: &'a str,
    pub attempts: usize,
    pub max_retries: usize,
    pub retry_interval: Duration,
}

pub struct ErrMaxRetryExceeded<'a, Context> {
    pub context: &'a Context,
    pub task_name: &'a str,
    pub max_retries: usize,
}

impl<'a, Context> Debug for ErrMaxRetryExceeded<'a, Context> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "operation failed after max retry of {}: {}",
            self.max_retries, self.task_name
        )
    }
}
