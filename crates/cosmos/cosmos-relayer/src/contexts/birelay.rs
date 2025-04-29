use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::Index;
use cgp::extra::run::RunnerComponent;
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::relayer_components::birelay::traits::AutoBiRelayerComponent;
use hermes_core::relayer_components::components::default::DefaultBiRelayComponents;
use hermes_core::relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::relay_at::{
    RelayGetterAtComponent, RelayTypeProviderAtComponent,
};
use hermes_core::runtime_components::traits::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_cosmos_core::tracing_logging_components::contexts::TracingLogger;
use hermes_prelude::*;
use hermes_runtime::types::runtime::HermesRuntime;

use crate::contexts::{CosmosChain, CosmosRelay};
use crate::impls::HandleCosmosError;

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
            ErrorTypeProviderComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeProviderComponent: UseType<HermesRuntime>,
        RuntimeGetterComponent: UseField<symbol!("runtime")>,
        LoggerComponent: TracingLogger,
        [
            ChainTypeProviderAtComponent<Index<0>>,
            ChainTypeProviderAtComponent<Index<1>>,
        ]: UseType<CosmosChain>,
        [
            RelayTypeProviderAtComponent<Index<0>, Index<1>>,
            RelayTypeProviderAtComponent<Index<1>, Index<0>>,
        ]: UseType<CosmosRelay>,
        RelayGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("relay_a_to_b")>,
        RelayGetterAtComponent<Index<1>, Index<0>>:
            UseField<symbol!("relay_b_to_a")>,
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

pub trait CanUseCosmosBiRelay:
    CanUseComponent<RunnerComponent> + CanUseComponent<AutoBiRelayerComponent>
{
}

impl CanUseCosmosBiRelay for CosmosBiRelay {}
