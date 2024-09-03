use cgp::core::Async;
use hermes_runtime_components::traits::runtime::ProvideRuntimeType;

use crate::types::runtime::HermesRuntime;

pub struct ProvideHermesRuntime;

impl<Context> ProvideRuntimeType<Context> for ProvideHermesRuntime
where
    Context: Async,
{
    type Runtime = HermesRuntime;
}
