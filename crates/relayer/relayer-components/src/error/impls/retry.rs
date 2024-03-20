use cgp_core::Async;

use crate::error::traits::retry::MaxErrorRetryGetter;

pub struct ReturnMaxRetry<const MAX_RETRY: usize>;

impl<Context, const MAX_RETRY: usize> MaxErrorRetryGetter<Context> for ReturnMaxRetry<MAX_RETRY>
where
    Context: Async,
{
    fn max_retry(_context: &Context) -> usize {
        MAX_RETRY
    }
}
