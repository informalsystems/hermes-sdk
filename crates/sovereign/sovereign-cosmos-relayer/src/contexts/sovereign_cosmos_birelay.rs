use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::birelay::traits::two_way::{
    ProvideTwoChainTypes, ProvideTwoWayRelayTypes, TwoWayRelayGetter,
};
use hermes_relayer_components::logger::traits::has_logger::LoggerFieldComponent;
use hermes_relayer_components::logger::traits::has_logger::LoggerTypeComponent;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;

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
        RuntimeTypeComponent: ProvideTokioRuntimeType,
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
    }
}

impl ProvideRuntime<SovereignCosmosBiRelay> for SovereignCosmosBiRelayComponents {
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
