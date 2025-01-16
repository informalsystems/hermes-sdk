use core::convert::Infallible;

use cgp::core::error::{ErrorRaiser, HasAsyncErrorType};

pub struct HandleInfallible;

impl<Context> ErrorRaiser<Context, Infallible> for HandleInfallible
where
    Context: HasAsyncErrorType,
{
    fn raise_error(e: Infallible) -> Context::Error {
        match e {}
    }
}
