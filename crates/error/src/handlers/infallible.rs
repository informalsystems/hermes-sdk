use core::convert::Infallible;

use cgp::core::error::{ErrorRaiser, ErrorRaiserComponent, HasAsyncErrorType};
use hermes_prelude::*;

pub struct HandleInfallible;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, Infallible> for HandleInfallible
where
    Context: HasAsyncErrorType,
{
    fn raise_error(e: Infallible) -> Context::Error {
        match e {}
    }
}
