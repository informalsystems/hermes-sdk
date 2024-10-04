use alloc::format;
use alloc::string::String;
use core::fmt::Debug;

use cgp::core::error::{ErrorRaiser, ErrorTypeComponent};
use cgp::core::types::impls::WithType;
use cgp::prelude::*;

pub struct ProvideStringError;

delegate_components! {
    ProvideStringError {
        ErrorTypeComponent: WithType<String>,
    }
}

impl<Context, Error> ErrorRaiser<Context, Error> for ProvideStringError
where
    Context: HasErrorType,
    Error: Debug,
    Context::Error: From<String>,
{
    fn raise_error(e: Error) -> Context::Error {
        format!("{:?}", e).into()
    }
}
