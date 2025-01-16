use cgp::core::error::{ErrorRaiser, HasAsyncErrorType};

use crate::types::Error;

pub struct ReturnError;

impl<Context> ErrorRaiser<Context, Error> for ReturnError
where
    Context: HasAsyncErrorType<Error = Error>,
{
    fn raise_error(e: Error) -> Error {
        e
    }
}
