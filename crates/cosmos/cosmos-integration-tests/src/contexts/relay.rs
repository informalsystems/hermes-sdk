use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;

use crate::contexts::chain::CosmosTestChain;

pub struct CosmosTestRelay {
    pub src_chain: CosmosTestChain,
    pub dst_chain: CosmosTestChain,
    pub base_relay: CosmosRelay,
}

pub struct CosmosTestRelayComponents;

delegate_components! {
    CosmosTestRelayComponents {
        ErrorTypeComponent:
            ProvideEyreError,
        ErrorRaiserComponent:
            RaiseDebugError,
    }
}
