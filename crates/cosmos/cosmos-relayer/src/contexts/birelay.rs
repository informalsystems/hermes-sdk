use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_relayer_components::birelay::traits::two_way::{
    ProvideTwoChainTypes, ProvideTwoWayRelayTypes, TwoWayRelayGetter,
};
use hermes_relayer_components::components::default::birelay::*;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    GetRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
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
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        RuntimeGetterComponent:
            GetRuntimeField<symbol!("runtime")>,
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

impl ProvideTwoChainTypes<CosmosBiRelay> for CosmosBiRelayComponents {
    type ChainA = CosmosChain;

    type ChainB = CosmosChain;
}

impl ProvideTwoWayRelayTypes<CosmosBiRelay> for CosmosBiRelayComponents {
    type RelayAToB = CosmosRelay;

    type RelayBToA = CosmosRelay;
}

impl TwoWayRelayGetter<CosmosBiRelay> for CosmosBiRelayComponents {
    fn relay_a_to_b(birelay: &CosmosBiRelay) -> &CosmosRelay {
        &birelay.relay_a_to_b
    }

    fn relay_b_to_a(birelay: &CosmosBiRelay) -> &CosmosRelay {
        &birelay.relay_b_to_a
    }
}
