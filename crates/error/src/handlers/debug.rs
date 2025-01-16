use alloc::sync::Arc;
use core::fmt::Debug;

use cgp::core::error::{ErrorRaiser, HasAsyncErrorType};
use eyre::eyre;

use crate::types::{Error, ErrorDetail};

pub struct DebugErrorWithRetry<const RETRYABLE: bool>;

pub type DebugRetryableError = DebugErrorWithRetry<true>;
pub type DebugError = DebugErrorWithRetry<false>;

impl<Context, E, const RETRYABLE: bool> ErrorRaiser<Context, E> for DebugErrorWithRetry<RETRYABLE>
where
    Context: HasAsyncErrorType<Error = Error>,
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        Error {
            is_retryable: RETRYABLE,
            detail: ErrorDetail::Report(Arc::new(eyre!("{:?}", e))),
        }
    }
}
