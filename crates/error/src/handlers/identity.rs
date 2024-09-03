use cgp::core::error::{ErrorRaiser, HasErrorType};

use crate::types::Error;

pub struct ReturnError;

impl<Context> ErrorRaiser<Context, Error> for ReturnError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: Error) -> Error {
        e
    }
}
