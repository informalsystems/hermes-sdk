use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::{Index, WithField};
use cgp::core::types::WithType;
use cgp::extra::run::CanRun;
use cgp::prelude::*;
use hermes_relayer_components::birelay::traits::two_way::{
    TwoWayRelayGetter, TwoWayRelayGetterComponent,
};
use hermes_relayer_components::components::default::birelay::{
    DefaultBiRelayComponents, IsDefaultBiRelayComponents,
};
use hermes_relayer_components::multi::traits::chain_at::ChainTypeAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayTypeAtComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};

use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::impls::error::HandleCosmosError;

#[cgp_context(CosmosBiRelayComponents: DefaultBiRelayComponents)]
#[derive(HasField, Clone)]
pub struct CosmosBiRelay {
    pub runtime: HermesRuntime,
    pub relay_a_to_b: CosmosRelay,
    pub relay_b_to_a: CosmosRelay,
}

delegate_components! {
    CosmosBiRelayComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        ChainTypeAtComponent<Index<0>>: WithType<CosmosChain>,
        ChainTypeAtComponent<Index<1>>: WithType<CosmosChain>,
        [
            RelayTypeAtComponent<Index<0>, Index<1>>,
            RelayTypeAtComponent<Index<1>, Index<0>>,
        ]: WithType<CosmosRelay>,
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

#[cgp_provider(TwoWayRelayGetterComponent)]
impl TwoWayRelayGetter<CosmosBiRelay> for CosmosBiRelayComponents {
    fn relay_a_to_b(birelay: &CosmosBiRelay) -> &CosmosRelay {
        &birelay.relay_a_to_b
    }

    fn relay_b_to_a(birelay: &CosmosBiRelay) -> &CosmosRelay {
        &birelay.relay_b_to_a
    }
}

pub trait CanUseCosmosBiRelay: CanRun {}

impl CanUseCosmosBiRelay for CosmosBiRelay {}
