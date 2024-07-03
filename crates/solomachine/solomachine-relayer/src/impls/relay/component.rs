use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_relayer_components::components::default::relay::*;
use hermes_relayer_components::with_default_relay_components;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeTypeComponent;

use crate::context::relay::SolomachineRelay;

pub struct SolomachineRelayComponents;

with_default_relay_components! {
    delegate_components! {
        SolomachineRelayComponents {
            @DefaultRelayComponents : DefaultRelayComponents,
        }
    }
}

impl<Chain> HasComponents for SolomachineRelay<Chain>
where
    Chain: Async,
{
    type Components = SolomachineRelayComponents;
}

delegate_components! {
    SolomachineRelayComponents {
        RuntimeTypeComponent: ProvideHermesRuntime,
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
    }
}
