use cgp_core::Async;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntimeType;

use crate::types::runtime::HermesRuntime;

pub struct ProvideHermesRuntime;

impl<Context> ProvideRuntimeType<Context> for ProvideHermesRuntime
where
    Context: Async,
{
    type Runtime = HermesRuntime;
}
