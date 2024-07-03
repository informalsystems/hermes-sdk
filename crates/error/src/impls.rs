use cgp_core::error::ProvideErrorType;
use cgp_core::Async;

use crate::types::Error;

pub struct ProvideHermesError;

impl<Context> ProvideErrorType<Context> for ProvideHermesError
where
    Context: Async,
{
    type Error = Error;
}
