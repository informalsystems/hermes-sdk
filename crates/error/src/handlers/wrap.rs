
use alloc::sync::Arc;
use core::fmt::Debug;

use cgp_core::error::{ErrorRaiser, HasErrorType};

use crate::types::{Error, ErrorDetail};

pub struct WrapErrorDetail;

impl<Context, Detail> ErrorRaiser<Context, (Detail, Error)> for WrapErrorDetail
where
    Context: HasErrorType<Error = Error>,
    Detail: Debug,
{
    fn raise_error((detail, e): (Detail, Error)) -> Error {
        Error {
            is_retryable: e.is_retryable,
            detail: ErrorDetail::Wrapped(format!("{:?}", detail), Arc::new(e.detail)),
        }
    }
}
