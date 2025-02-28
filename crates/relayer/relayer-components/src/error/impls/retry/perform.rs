use core::fmt::Debug;
use core::time::Duration;

use cgp::extra::runtime::HasRuntime;
use cgp::prelude::*;
use hermes_runtime_components::traits::sleep::CanSleep;

use crate::error::traits::{Cont, HasRetryableError, RetryPerformer, RetryPerformerComponent};

#[cgp_new_provider(RetryPerformerComponent)]
impl<Context> RetryPerformer<Context> for PerformRetryWithRetryableError
where
    Context: HasRuntime
        + HasRetryableError
        + for<'a> CanWrapAsyncError<ErrMaxRetryExceeded<'a, Context>>,
    Context::Runtime: CanSleep,
{
    async fn perform_retry<T: Send + Sync>(
        context: &Context,
        task_name: &str,
        num_retries: usize,
        cont: impl Cont<Result<T, Context::Error>>,
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
                    } else if attempts >= num_retries {
                        return Err(Context::wrap_error(
                            e,
                            ErrMaxRetryExceeded {
                                context,
                                task_name,
                                num_retries,
                            },
                        ));
                    } else {
                        runtime.sleep(retry_interval).await;
                        attempts += 1;
                        retry_interval = retry_interval * 2;
                    }
                }
            }
        }
    }
}

pub struct ErrMaxRetryExceeded<'a, Context> {
    pub context: &'a Context,
    pub task_name: &'a str,
    pub num_retries: usize,
}

impl<'a, Context> Debug for ErrMaxRetryExceeded<'a, Context> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "operation failed after max retry of {}: {}",
            self.num_retries, self.task_name
        )
    }
}
