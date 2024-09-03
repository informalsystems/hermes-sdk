use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_relayer_components::birelay::traits::two_way::TwoWayRelayGetter;
use hermes_relayer_components::components::default::birelay::*;
use hermes_relayer_components::multi::traits::chain_at::ProvideChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::ProvideRelayTypeAt;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};

use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::impls::error::HandleCosmosError;

#[derive(HasField, Clone)]
pub struct CosmosBiRelay {
    pub runtime: HermesRuntime,
    pub relay_a_to_b: CosmosRelay,
    pub relay_b_to_a: CosmosRelay,
}

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
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
    }
}

impl CosmosBiRelay {
    pub fn new(
        runtime: HermesRuntime,
        relay_a_to_b: CosmosRelay,
        relay_b_to_a: CosmosRelay,
    ) -> Self {
        Self {
            runtime,
            relay_a_to_b,
            relay_b_to_a,
        }
    }
}

impl ProvideChainTypeAt<CosmosBiRelay, 0> for CosmosBiRelayComponents {
    type Chain = CosmosChain;
}

impl ProvideChainTypeAt<CosmosBiRelay, 1> for CosmosBiRelayComponents {
    type Chain = CosmosChain;
}

impl ProvideRelayTypeAt<CosmosBiRelay, 0, 1> for CosmosBiRelayComponents {
    type Relay = CosmosRelay;
}

impl ProvideRelayTypeAt<CosmosBiRelay, 1, 0> for CosmosBiRelayComponents {
    type Relay = CosmosRelay;
}

impl TwoWayRelayGetter<CosmosBiRelay> for CosmosBiRelayComponents {
    fn relay_a_to_b(birelay: &CosmosBiRelay) -> &CosmosRelay {
        &birelay.relay_a_to_b
    }

    fn relay_b_to_a(birelay: &CosmosBiRelay) -> &CosmosRelay {
        &birelay.relay_b_to_a
    }
}
