use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_relayer::chain::impls::create_client_message::DelegateCosmosCreateClientMessageBuilder;
use hermes_cosmos_relayer::chain::impls::update_client_message::DelegateCosmosUpdateClientMessageBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::types::client_state::{
    CanDecodeClientState, HasClientStateType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_relayer_components::encode::traits::has_encoding::{
    EncodingGetterComponent, EncodingTypeComponent,
};
use hermes_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use hermes_relayer_components::runtime::traits::runtime::{ProvideRuntime, RuntimeTypeComponent};
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_client_components::cosmos::impls::client::create_client_message::BuildCreateSovereignClientMessageOnCosmos;
use hermes_sovereign_client_components::cosmos::impls::client::update_client_message::BuildUpdateSovereignClientMessageOnCosmos;
use hermes_sovereign_client_components::sovereign::components::chain::{
    IsSovereignChainClientComponent, SovereignChainClientComponents,
};
use hermes_sovereign_client_components::sovereign::traits::chain::data_chain::{
    DataChainGetter, DataChainGetterComponent, DataChainTypeComponent, HasDataChain,
    ProvideDataChainType,
};
use hermes_sovereign_client_components::sovereign::types::client_state::SovereignClientState;
use hermes_sovereign_client_components::sovereign::types::height::RollupHeight;

use crate::contexts::encoding::ProvideSovereignEncoding;

pub struct SovereignChain {
    pub runtime: HermesRuntime,
    pub data_chain: CosmosChain,
    // TODO: fields such as rollup JSON RPC address
}

pub struct SovereignChainComponents;

impl HasComponents for SovereignChain {
    type Components = SovereignChainComponents;
}

delegate_all!(
    IsSovereignChainClientComponent,
    SovereignChainClientComponents,
    SovereignChainComponents,
);

pub struct SovereignDataChainType;

impl<Chain> ProvideDataChainType<Chain> for SovereignDataChainType
where
    Chain: Async,
{
    type DataChain = CosmosChain;
}

impl DataChainGetter<SovereignChain> for SovereignDataChainType {
    fn data_chain(chain: &SovereignChain) -> &CosmosChain {
        &chain.data_chain
    }
}

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
        [
            DataChainTypeComponent,
            DataChainGetterComponent,
        ]: SovereignDataChainType,
        [
            EncodingTypeComponent,
            EncodingGetterComponent,
        ]:
            ProvideSovereignEncoding,
    }
}

delegate_components! {
    DelegateCosmosCreateClientMessageBuilder {
        SovereignChain: BuildCreateSovereignClientMessageOnCosmos,
    }
}

delegate_components! {
    DelegateCosmosUpdateClientMessageBuilder {
        SovereignChain: BuildUpdateSovereignClientMessageOnCosmos,
    }
}

impl ProvideRuntime<SovereignChain> for SovereignChainComponents {
    fn runtime(chain: &SovereignChain) -> &HermesRuntime {
        &chain.runtime
    }
}

pub trait CheckSovereignChainImpls:
    HasDataChain
    + HasUpdateClientPayloadType<CosmosChain>
    + HasHeightType<Height = RollupHeight>
    + HasClientStateType<CosmosChain, ClientState = SovereignClientState>
    + CanBuildUpdateClientPayload<CosmosChain>
    + CanDecodeClientState<CosmosChain>
{
}

impl CheckSovereignChainImpls for SovereignChain {}

pub trait CheckCosmosChainImpls:
    CanQueryClientState<SovereignChain>
    + CanBuildCreateClientMessage<SovereignChain>
    + CanBuildUpdateClientMessage<SovereignChain>
{
}

impl CheckCosmosChainImpls for CosmosChain {}
