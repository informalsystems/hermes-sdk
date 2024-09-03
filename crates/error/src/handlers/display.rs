use alloc::sync::Arc;
use core::fmt::Display;

use cgp::core::error::{ErrorRaiser, HasErrorType};
use eyre::eyre;

use crate::types::{Error, ErrorDetail};

pub struct DisplayErrorWithRetry<const RETRYABLE: bool>;

pub type DisplayRetryableError = DisplayErrorWithRetry<true>;
pub type DisplayError = DisplayErrorWithRetry<false>;

impl<Context, E, const RETRYABLE: bool> ErrorRaiser<Context, E> for DisplayErrorWithRetry<RETRYABLE>
where
    Context: HasErrorType<Error = Error>,
    E: Display,
{
    fn raise_error(e: E) -> Error {
        Error {
            is_retryable: RETRYABLE,
            detail: ErrorDetail::Report(Arc::new(eyre!("{}", e))),
        }
    }
}
