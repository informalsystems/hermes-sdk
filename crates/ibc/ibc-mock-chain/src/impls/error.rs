use alloc::format;
use alloc::string::String;
use core::fmt::Debug;

use cgp::core::error::{ErrorRaiser, ErrorRaiserComponent};
use hermes_prelude::*;

pub struct RaiseDebugString;

#[cgp_provider(ErrorRaiserComponent)]
impl<Context, Error> ErrorRaiser<Context, Error> for RaiseDebugString
where
    Context: HasAsyncErrorType,
    Error: Debug,
    Context::Error: From<String>,
{
    fn raise_error(e: Error) -> Context::Error {
        format!("{:?}", e).into()
    }
}
