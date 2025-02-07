use cgp::prelude::*;
use hermes_runtime_components::traits::runtime::{ProvideRuntimeType, RuntimeTypeComponent};

use crate::types::runtime::HermesRuntime;

pub struct ProvideHermesRuntime;

#[cgp_provider(RuntimeTypeComponent)]
impl<Context> ProvideRuntimeType<Context> for ProvideHermesRuntime
where
    Context: Async,
{
    type Runtime = HermesRuntime;
}
