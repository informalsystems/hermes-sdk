use alloc::format;
use alloc::sync::Arc;
use core::fmt::Debug;

use cgp::core::error::{ErrorWrapper, ErrorWrapperComponent, HasAsyncErrorType};
use hermes_prelude::*;

use crate::types::{Error, ErrorDetail};

pub struct WrapErrorDetail;

#[cgp_provider(ErrorWrapperComponent)]
impl<Context, Detail> ErrorWrapper<Context, Detail> for WrapErrorDetail
where
    Context: HasAsyncErrorType<Error = Error>,
    Detail: Debug,
{
    fn wrap_error(error: Error, detail: Detail) -> Error {
        Error {
            is_retryable: error.is_retryable,
            detail: ErrorDetail::Wrapped(format!("{detail:?}"), Arc::new(error.detail)),
        }
    }
}
