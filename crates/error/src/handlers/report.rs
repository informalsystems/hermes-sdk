use alloc::sync::Arc;

use cgp::core::error::{ErrorRaiser, HasAsyncErrorType};
use eyre::Report;

use crate::types::{Error, ErrorDetail};

pub struct ReportErrorWithRetry<const RETRYABLE: bool>;

pub type ReportRetryableError = ReportErrorWithRetry<true>;
pub type ReportError = ReportErrorWithRetry<false>;

impl<Context, E, const RETRYABLE: bool> ErrorRaiser<Context, E> for ReportErrorWithRetry<RETRYABLE>
where
    Context: HasAsyncErrorType<Error = Error>,
    Report: From<E>,
{
    fn raise_error(e: E) -> Error {
        Error {
            is_retryable: RETRYABLE,
            detail: ErrorDetail::Report(Arc::new(e.into())),
        }
    }
}
