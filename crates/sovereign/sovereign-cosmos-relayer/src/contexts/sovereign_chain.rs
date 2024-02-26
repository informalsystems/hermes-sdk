use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_relayer::chain::impls::create_client_message::DelegateCosmosCreateClientMessageBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use hermes_relayer_components::runtime::traits::runtime::{ProvideRuntime, RuntimeTypeComponent};
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_client_components::cosmos::impls::client::create_client_message::BuildCreateSovereignClientMessageOnCosmos;
use hermes_sovereign_client_components::sovereign::components::{
    IsSovereignClientComponent, SovereignClientComponents,
};
use hermes_sovereign_client_components::sovereign::traits::data_chain::HasDataChain;

pub struct SovereignChain {
    pub runtime: HermesRuntime,
    pub data_chain: CosmosChain,
    // TODO: fields such as rollup JSON RPC address
}

pub struct SovereignChainComponents;

impl HasComponents for SovereignChain {
    type Components = SovereignChainComponents;
}

impl HasDataChain for SovereignChain {
    type DataChain = CosmosChain;

    fn data_chain(&self) -> &CosmosChain {
        &self.data_chain
    }
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

delegate_components! {
    DelegateCosmosCreateClientMessageBuilder {
        SovereignChain: BuildCreateSovereignClientMessageOnCosmos,
    }
}

impl ProvideRuntime<SovereignChain> for SovereignChainComponents {
    fn runtime(chain: &SovereignChain) -> &HermesRuntime {
        &chain.runtime
    }
}

pub trait CheckCanBuildCreateClientMessage: CanBuildCreateClientMessage<SovereignChain> {}

impl CheckCanBuildCreateClientMessage for CosmosChain {}
