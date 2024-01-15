use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;

pub struct CosmosRelayDriver {
    pub base_relay: CosmosRelay,
}

pub struct CosmosRelayDriverComponents;

delegate_components! {
    CosmosRelayDriverComponents {
        ErrorTypeComponent:
            ProvideEyreError,
        ErrorRaiserComponent:
            RaiseDebugError,
    }
}
