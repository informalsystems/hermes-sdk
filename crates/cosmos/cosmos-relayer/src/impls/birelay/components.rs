use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use hermes_relayer_components::components::default::birelay::{
    DefaultBiRelayComponents, IsDefaultBiRelayComponent,
};
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeTypeComponent;

use crate::contexts::birelay::CosmosBiRelay;
use crate::impls::error::HandleCosmosError;

pub struct CosmosBiRelayComponents;

impl HasComponents for CosmosBiRelay {
    type Components = CosmosBiRelayComponents;
}

delegate_all!(
    IsDefaultBiRelayComponent,
    DefaultBiRelayComponents,
    CosmosBiRelayComponents,
);

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
