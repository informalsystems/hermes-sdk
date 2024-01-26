use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use hermes_celestia_chain::contexts::celestia_chain::CelestiaChain;
use hermes_relayer_components::logger::traits::has_logger::LoggerFieldComponent;
use hermes_relayer_components::logger::traits::has_logger::LoggerTypeComponent;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_client_components::impls::sovereign::components::IsSovereignClientComponent;
use hermes_sovereign_client_components::impls::sovereign::components::SovereignClientComponents;

pub struct SovereignChain {
    pub runtime: HermesRuntime,
    pub celestia_chain: CelestiaChain,
    // TODO: fields such as rollup JSON RPC address
}

pub struct SovereignChainComponents;

impl HasComponents for SovereignChain {
    type Components = SovereignChainComponents;
}

delegate_all!(
    IsSovereignClientComponent,
    SovereignClientComponents,
    SovereignChainComponents,
);

delegate_components! {
    SovereignChainComponents {
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

impl ProvideRuntime<SovereignChain> for SovereignChainComponents {
    fn runtime(chain: &SovereignChain) -> &HermesRuntime {
        &chain.runtime
    }
}
