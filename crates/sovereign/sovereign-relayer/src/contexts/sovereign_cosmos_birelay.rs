use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::birelay::traits::two_way::{
    ProvideTwoChainTypes, ProvideTwoWayRelayTypes, TwoWayRelayGetter,
};
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};

use crate::contexts::cosmos_to_sovereign_relay::CosmosToSovereignRelay;
use crate::contexts::sovereign_chain::SovereignChain;
use crate::contexts::sovereign_to_cosmos_relay::SovereignToCosmosRelay;

pub struct SovereignCosmosBiRelay {
    pub runtime: HermesRuntime,
    pub relay_a_to_b: CosmosToSovereignRelay,
    pub relay_b_to_a: SovereignToCosmosRelay,
}

pub struct SovereignCosmosBiRelayComponents;

impl HasComponents for SovereignCosmosBiRelay {
    type Components = SovereignCosmosBiRelayComponents;
}

delegate_components! {
    SovereignCosmosBiRelayComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        RuntimeTypeComponent: ProvideHermesRuntime,
    }
}

impl RuntimeGetter<SovereignCosmosBiRelay> for SovereignCosmosBiRelayComponents {
    fn runtime(relay: &SovereignCosmosBiRelay) -> &HermesRuntime {
        &relay.runtime
    }
}

impl ProvideTwoChainTypes<SovereignCosmosBiRelay> for SovereignCosmosBiRelayComponents {
    type ChainA = CosmosChain;

    type ChainB = SovereignChain;
}

impl ProvideTwoWayRelayTypes<SovereignCosmosBiRelay> for SovereignCosmosBiRelayComponents {
    type RelayAToB = CosmosToSovereignRelay;

    type RelayBToA = SovereignToCosmosRelay;
}

impl TwoWayRelayGetter<SovereignCosmosBiRelay> for SovereignCosmosBiRelayComponents {
    fn relay_a_to_b(birelay: &SovereignCosmosBiRelay) -> &CosmosToSovereignRelay {
        &birelay.relay_a_to_b
    }

    fn relay_b_to_a(birelay: &SovereignCosmosBiRelay) -> &SovereignToCosmosRelay {
        &birelay.relay_b_to_a
    }
}
