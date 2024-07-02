use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_relayer_components::components::default::birelay::*;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeTypeComponent;

use crate::contexts::birelay::CosmosBiRelay;
use crate::impls::error::HandleCosmosError;

pub struct CosmosBiRelayComponents;

impl HasComponents for CosmosBiRelay {
    type Components = CosmosBiRelayComponents;
}

with_default_bi_relay_components! {
    delegate_components! {
        CosmosBiRelayComponents {
            @DefaultBiRelayComponents: DefaultBiRelayComponents,
        }
    }
}

delegate_components! {
    CosmosBiRelayComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
    }
}
