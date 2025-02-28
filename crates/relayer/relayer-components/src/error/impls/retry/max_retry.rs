use cgp::prelude::*;

use crate::error::traits::{MaxErrorRetryGetter, MaxErrorRetryGetterComponent};

pub struct ReturnMaxRetry<const MAX_RETRY: usize>;

#[cgp_provider(MaxErrorRetryGetterComponent)]
impl<Context, const MAX_RETRY: usize> MaxErrorRetryGetter<Context> for ReturnMaxRetry<MAX_RETRY>
where
    Context: Async,
{
    fn max_retry(_context: &Context) -> usize {
        MAX_RETRY
    }
}
