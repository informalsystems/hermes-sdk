use alloc::format;
use alloc::sync::Arc;
use core::fmt::Debug;

use cgp_core::error::{ErrorRaiser, HasErrorType};

use crate::traits::wrap::WrapError;
use crate::types::{Error, ErrorDetail};

pub struct WrapErrorDetail;

impl<Context, Detail> ErrorRaiser<Context, WrapError<Detail, Error>> for WrapErrorDetail
where
    Context: HasErrorType<Error = Error>,
    Detail: Debug,
{
    fn raise_error(WrapError { detail, error }: WrapError<Detail, Error>) -> Error {
        Error {
            is_retryable: error.is_retryable,
            detail: ErrorDetail::Wrapped(format!("{:?}", detail), Arc::new(error.detail)),
        }
    }
}
