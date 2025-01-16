use cgp::core::error::{CanRaiseAsyncError, HasAsyncErrorType};

pub struct WrapError<Detail, Error> {
    pub detail: Detail,
    pub error: Error,
}

pub trait CanWrapError<Detail>: HasAsyncErrorType {
    fn wrap_error(detail: Detail, error: Self::Error) -> Self::Error;
}

impl<Context, Detail, Error> CanWrapError<Detail> for Context
where
    Context: HasAsyncErrorType<Error = Error> + CanRaiseAsyncError<WrapError<Detail, Error>>,
{
    fn wrap_error(detail: Detail, error: Self::Error) -> Self::Error {
        Context::raise_error(WrapError { detail, error })
    }
}
