use alloc::format;
use alloc::string::String;
use core::fmt::Debug;

use cgp::core::error::ErrorRaiser;
use cgp::prelude::*;

pub struct RaiseDebugString;

impl<Context, Error> ErrorRaiser<Context, Error> for RaiseDebugString
where
    Context: HasErrorType,
    Error: Debug,
    Context::Error: From<String>,
{
    fn raise_error(e: Error) -> Context::Error {
        format!("{:?}", e).into()
    }
}
