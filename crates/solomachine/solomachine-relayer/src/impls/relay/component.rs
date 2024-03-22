use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use hermes_cosmos_relayer::types::error::{DebugError, ProvideCosmosError};
use hermes_relayer_components::components::default::relay::{
    DefaultRelayComponents, IsDefaultRelayComponent,
};
use hermes_relayer_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeTypeComponent;

use crate::context::relay::SolomachineRelay;

pub struct SolomachineRelayComponents;

delegate_all!(
    IsDefaultRelayComponent,
    DefaultRelayComponents,
    SolomachineRelayComponents,
);

impl<Chain> HasComponents for SolomachineRelay<Chain>
where
    Chain: Async,
{
    type Components = SolomachineRelayComponents;
}

delegate_components! {
    SolomachineRelayComponents {
        RuntimeTypeComponent: ProvideHermesRuntime,
        ErrorTypeComponent: ProvideCosmosError,
        ErrorRaiserComponent: DebugError,
    }
}
