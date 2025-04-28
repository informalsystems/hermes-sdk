use cgp::prelude::*;
use hermes_runtime_components::traits::{RuntimeTypeProvider, RuntimeTypeProviderComponent};

use crate::types::runtime::HermesRuntime;

pub struct ProvideHermesRuntime;

#[cgp_provider(RuntimeTypeProviderComponent)]
impl<Context> RuntimeTypeProvider<Context> for ProvideHermesRuntime
where
    Context: Async,
{
    type Runtime = HermesRuntime;
}
