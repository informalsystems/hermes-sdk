use core::convert::Infallible;

use cgp_core::error::{ErrorRaiser, HasErrorType};

pub struct HandleInfallible;

impl<Context> ErrorRaiser<Context, Infallible> for HandleInfallible
where
    Context: HasErrorType,
{
    fn raise_error(e: Infallible) -> Context::Error {
        match e {}
    }
}
