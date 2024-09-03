use cgp::core::error::{CanRaiseError, HasErrorType};

pub struct WrapError<Detail, Error> {
    pub detail: Detail,
    pub error: Error,
}

pub trait CanWrapError<Detail>: HasErrorType {
    fn wrap_error(detail: Detail, error: Self::Error) -> Self::Error;
}

impl<Context, Detail, Error> CanWrapError<Detail> for Context
where
    Context: HasErrorType<Error = Error> + CanRaiseError<WrapError<Detail, Error>>,
{
    fn wrap_error(detail: Detail, error: Self::Error) -> Self::Error {
        Context::raise_error(WrapError { detail, error })
    }
}
