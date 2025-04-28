use cgp::core::error::{ErrorRaiser, ErrorRaiserComponent, HasAsyncErrorType};
use hermes_prelude::*;

use crate::types::Error;

pub struct ReturnError;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, Error> for ReturnError
where
    Context: HasAsyncErrorType<Error = Error>,
{
    fn raise_error(e: Error) -> Error {
        e
    }
}
