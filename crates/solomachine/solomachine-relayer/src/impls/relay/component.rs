use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use hermes_cosmos_relayer::types::error::DebugError;
use hermes_cosmos_relayer::types::error::ProvideCosmosError;
use hermes_relayer_components::components::default::relay::{
    DefaultRelayComponents, IsDefaultRelayComponent,
};
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;

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
        RuntimeTypeComponent: ProvideTokioRuntimeType,
        ErrorTypeComponent: ProvideCosmosError,
        ErrorRaiserComponent: DebugError,
    }
}
